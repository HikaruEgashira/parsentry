You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-017
- **Kind**: file_handler
- **Identifier**: Multi-backend File Storage (S3, Azure, GCS, Aliyun, Tencent, Supabase)
- **Description**: File storage abstraction with multiple cloud backends. Misconfigured storage credentials, path traversal in storage keys, and insecure direct object reference to stored files
- **Locations**: api/extensions/storage/aws_s3_storage.py, api/extensions/storage/azure_blob_storage.py, api/extensions/storage/google_cloud_storage.py, api/extensions/storage/aliyun_oss_storage.py, api/extensions/storage/tencent_cos_storage.py, api/extensions/storage/supabase_storage.py, api/extensions/storage/opendal_storage.py

## Source Code

### api/extensions/storage/aws_s3_storage.py
```py
import logging
from collections.abc import Generator

import boto3
from botocore.client import Config
from botocore.exceptions import ClientError

from configs import dify_config
from extensions.storage.base_storage import BaseStorage

logger = logging.getLogger(__name__)


class AwsS3Storage(BaseStorage):
    """Implementation for Amazon Web Services S3 storage."""

    def __init__(self):
        super().__init__()
        self.bucket_name = dify_config.S3_BUCKET_NAME
        if dify_config.S3_USE_AWS_MANAGED_IAM:
            logger.info("Using AWS managed IAM role for S3")

            session = boto3.Session()
            region_name = dify_config.S3_REGION
            self.client = session.client(service_name="s3", region_name=region_name)
        else:
            logger.info("Using ak and sk for S3")

            self.client = boto3.client(
                "s3",
                aws_secret_access_key=dify_config.S3_SECRET_KEY,
                aws_access_key_id=dify_config.S3_ACCESS_KEY,
                endpoint_url=dify_config.S3_ENDPOINT,
                region_name=dify_config.S3_REGION,
                config=Config(s3={"addressing_style": dify_config.S3_ADDRESS_STYLE}),
            )
        # create bucket
        try:
            self.client.head_bucket(Bucket=self.bucket_name)
        except ClientError as e:
            # if bucket not exists, create it
            if e.response.get("Error", {}).get("Code") == "404":
                self.client.create_bucket(Bucket=self.bucket_name)
            # if bucket is not accessible, pass, maybe the bucket is existing but not accessible
            elif e.response.get("Error", {}).get("Code") == "403":
                pass
            else:
                # other error, raise exception
                raise

    def save(self, filename, data):
        self.client.put_object(Bucket=self.bucket_name, Key=filename, Body=data)

    def load_once(self, filename: str) -> bytes:
        try:
            data: bytes = self.client.get_object(Bucket=self.bucket_name, Key=filename)["Body"].read()
        except ClientError as ex:
            if ex.response.get("Error", {}).get("Code") == "NoSuchKey":
                raise FileNotFoundError("File not found")
            else:
                raise
        return data

    def load_stream(self, filename: str) -> Generator:
        try:
            response = self.client.get_object(Bucket=self.bucket_name, Key=filename)
            yield from response["Body"].iter_chunks()
        except ClientError as ex:
            if ex.response.get("Error", {}).get("Code") == "NoSuchKey":
                raise FileNotFoundError("file not found")
            elif "reached max retries" in str(ex):
                raise ValueError("please do not request the same file too frequently")
            else:
                raise

    def download(self, filename, target_filepath):
        self.client.download_file(self.bucket_name, filename, target_filepath)

    def exists(self, filename):
        try:
            self.client.head_object(Bucket=self.bucket_name, Key=filename)
            return True
        except:
            return False

    def delete(self, filename: str):
        self.client.delete_object(Bucket=self.bucket_name, Key=filename)

```

### api/extensions/storage/azure_blob_storage.py
```py
from collections.abc import Generator
from datetime import timedelta

from azure.identity import ChainedTokenCredential, DefaultAzureCredential
from azure.storage.blob import AccountSasPermissions, BlobServiceClient, ResourceTypes, generate_account_sas

from configs import dify_config
from extensions.ext_redis import redis_client
from extensions.storage.base_storage import BaseStorage
from libs.datetime_utils import naive_utc_now


class AzureBlobStorage(BaseStorage):
    """Implementation for Azure Blob storage."""

    def __init__(self):
        super().__init__()
        self.bucket_name = dify_config.AZURE_BLOB_CONTAINER_NAME
        self.account_url = dify_config.AZURE_BLOB_ACCOUNT_URL
        self.account_name = dify_config.AZURE_BLOB_ACCOUNT_NAME
        self.account_key = dify_config.AZURE_BLOB_ACCOUNT_KEY

        self.credential: ChainedTokenCredential | None = None
        if self.account_key == "managedidentity":
            self.credential = DefaultAzureCredential()
        else:
            self.credential = None

    def save(self, filename, data):
        if not self.bucket_name:
            return

        client = self._sync_client()
        blob_container = client.get_container_client(container=self.bucket_name)
        blob_container.upload_blob(filename, data)

    def load_once(self, filename: str) -> bytes:
        if not self.bucket_name:
            raise FileNotFoundError("Azure bucket name is not configured.")

        client = self._sync_client()
        blob = client.get_container_client(container=self.bucket_name)
        blob = blob.get_blob_client(blob=filename)
        data = blob.download_blob().readall()
        if not isinstance(data, bytes):
            raise TypeError(f"Expected bytes from blob.readall(), got {type(data).__name__}")
        return data

    def load_stream(self, filename: str) -> Generator:
        if not self.bucket_name:
            raise FileNotFoundError("Azure bucket name is not configured.")

        client = self._sync_client()
        blob = client.get_blob_client(container=self.bucket_name, blob=filename)
        blob_data = blob.download_blob()
        yield from blob_data.chunks()

    def download(self, filename, target_filepath):
        if not self.bucket_name:
            return

        client = self._sync_client()

        blob = client.get_blob_client(container=self.bucket_name, blob=filename)
        with open(target_filepath, "wb") as my_blob:
            blob_data = blob.download_blob()
            blob_data.readinto(my_blob)

    def exists(self, filename):
        if not self.bucket_name:
            return False

        client = self._sync_client()

        blob = client.get_blob_client(container=self.bucket_name, blob=filename)
        return blob.exists()

    def delete(self, filename: str):
        if not self.bucket_name:
            return

        client = self._sync_client()

        blob_container = client.get_container_client(container=self.bucket_name)
        blob_container.delete_blob(filename)

    def _sync_client(self):
        if self.account_key == "managedidentity":
            return BlobServiceClient(account_url=self.account_url, credential=self.credential)  # type: ignore

        cache_key = f"azure_blob_sas_token_{self.account_name}_{self.account_key}"
        cache_result = redis_client.get(cache_key)
        if cache_result is not None:
            sas_token = cache_result.decode("utf-8")
        else:
            sas_token = generate_account_sas(
                account_name=self.account_name or "",
                account_key=self.account_key or "",
                resource_types=ResourceTypes(service=True, container=True, object=True),
                permission=AccountSasPermissions(read=True, write=True, delete=True, list=True, add=True, create=True),
                expiry=naive_utc_now() + timedelta(hours=1),
            )
            redis_client.set(cache_key, sas_token, ex=3000)
        return BlobServiceClient(account_url=self.account_url or "", credential=sas_token)

```

### api/extensions/storage/google_cloud_storage.py
```py
import base64
import io
from collections.abc import Generator
from typing import Any

from google.cloud import storage as google_cloud_storage  # type: ignore
from pydantic import TypeAdapter

from configs import dify_config
from extensions.storage.base_storage import BaseStorage

_service_account_adapter: TypeAdapter[dict[str, Any]] = TypeAdapter(dict[str, Any])


class GoogleCloudStorage(BaseStorage):
    """Implementation for Google Cloud storage."""

    def __init__(self):
        super().__init__()

        self.bucket_name = dify_config.GOOGLE_STORAGE_BUCKET_NAME
        service_account_json_str = dify_config.GOOGLE_STORAGE_SERVICE_ACCOUNT_JSON_BASE64
        # if service_account_json_str is empty, use Application Default Credentials
        if service_account_json_str:
            service_account_json = base64.b64decode(service_account_json_str).decode("utf-8")
            # convert str to object
            service_account_obj = _service_account_adapter.validate_json(service_account_json)
            self.client = google_cloud_storage.Client.from_service_account_info(service_account_obj)
        else:
            self.client = google_cloud_storage.Client()

    def save(self, filename, data):
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.blob(filename)
        with io.BytesIO(data) as stream:
            blob.upload_from_file(stream)

    def load_once(self, filename: str) -> bytes:
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.get_blob(filename)
        if blob is None:
            raise FileNotFoundError("File not found")
        data: bytes = blob.download_as_bytes()
        return data

    def load_stream(self, filename: str) -> Generator:
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.get_blob(filename)
        if blob is None:
            raise FileNotFoundError("File not found")
        with blob.open(mode="rb") as blob_stream:
            while chunk := blob_stream.read(4096):
                yield chunk

    def download(self, filename, target_filepath):
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.get_blob(filename)
        if blob is None:
            raise FileNotFoundError("File not found")
        blob.download_to_filename(target_filepath)

    def exists(self, filename):
        bucket = self.client.get_bucket(self.bucket_name)
        blob = bucket.blob(filename)
        return blob.exists()

    def delete(self, filename: str):
        bucket = self.client.get_bucket(self.bucket_name)
        bucket.delete_blob(filename)

```

### api/extensions/storage/aliyun_oss_storage.py
```py
import posixpath
from collections.abc import Generator

import oss2 as aliyun_s3

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class AliyunOssStorage(BaseStorage):
    """Implementation for Aliyun OSS storage."""

    def __init__(self):
        super().__init__()
        self.bucket_name = dify_config.ALIYUN_OSS_BUCKET_NAME
        self.folder = dify_config.ALIYUN_OSS_PATH
        oss_auth_method = aliyun_s3.Auth
        region = None
        if dify_config.ALIYUN_OSS_AUTH_VERSION == "v4":
            oss_auth_method = aliyun_s3.AuthV4
            region = dify_config.ALIYUN_OSS_REGION
        oss_auth = oss_auth_method(dify_config.ALIYUN_OSS_ACCESS_KEY, dify_config.ALIYUN_OSS_SECRET_KEY)
        self.client = aliyun_s3.Bucket(
            oss_auth,
            dify_config.ALIYUN_OSS_ENDPOINT,
            self.bucket_name,
            connect_timeout=30,
            region=region,
            cloudbox_id=dify_config.ALIYUN_CLOUDBOX_ID,
        )

    def save(self, filename, data):
        self.client.put_object(self.__wrapper_folder_filename(filename), data)

    def load_once(self, filename: str) -> bytes:
        obj = self.client.get_object(self.__wrapper_folder_filename(filename))
        data = obj.read()
        if not isinstance(data, bytes):
            return b""
        return data

    def load_stream(self, filename: str) -> Generator:
        obj = self.client.get_object(self.__wrapper_folder_filename(filename))
        while chunk := obj.read(4096):
            yield chunk

    def download(self, filename: str, target_filepath):
        self.client.get_object_to_file(self.__wrapper_folder_filename(filename), target_filepath)

    def exists(self, filename: str):
        return self.client.object_exists(self.__wrapper_folder_filename(filename))

    def delete(self, filename: str):
        self.client.delete_object(self.__wrapper_folder_filename(filename))

    def __wrapper_folder_filename(self, filename: str) -> str:
        return posixpath.join(self.folder, filename) if self.folder else filename

```

### api/extensions/storage/tencent_cos_storage.py
```py
from collections.abc import Generator

from qcloud_cos import CosConfig, CosS3Client

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class TencentCosStorage(BaseStorage):
    """Implementation for Tencent Cloud COS storage."""

    def __init__(self):
        super().__init__()

        self.bucket_name = dify_config.TENCENT_COS_BUCKET_NAME
        if dify_config.TENCENT_COS_CUSTOM_DOMAIN:
            config = CosConfig(
                Domain=dify_config.TENCENT_COS_CUSTOM_DOMAIN,
                SecretId=dify_config.TENCENT_COS_SECRET_ID,
                SecretKey=dify_config.TENCENT_COS_SECRET_KEY,
                Scheme=dify_config.TENCENT_COS_SCHEME,
            )
        else:
            config = CosConfig(
                Region=dify_config.TENCENT_COS_REGION,
                SecretId=dify_config.TENCENT_COS_SECRET_ID,
                SecretKey=dify_config.TENCENT_COS_SECRET_KEY,
                Scheme=dify_config.TENCENT_COS_SCHEME,
            )
        self.client = CosS3Client(config)

    def save(self, filename, data):
        self.client.put_object(Bucket=self.bucket_name, Body=data, Key=filename)

    def load_once(self, filename: str) -> bytes:
        data: bytes = self.client.get_object(Bucket=self.bucket_name, Key=filename)["Body"].get_raw_stream().read()
        return data

    def load_stream(self, filename: str) -> Generator:
        response = self.client.get_object(Bucket=self.bucket_name, Key=filename)
        yield from response["Body"].get_stream(chunk_size=4096)

    def download(self, filename, target_filepath):
        response = self.client.get_object(Bucket=self.bucket_name, Key=filename)
        response["Body"].get_stream_to_file(target_filepath)

    def exists(self, filename):
        return self.client.object_exists(Bucket=self.bucket_name, Key=filename)

    def delete(self, filename: str):
        self.client.delete_object(Bucket=self.bucket_name, Key=filename)

```

### api/extensions/storage/supabase_storage.py
```py
import io
from collections.abc import Generator
from pathlib import Path

from supabase import Client

from configs import dify_config
from extensions.storage.base_storage import BaseStorage


class SupabaseStorage(BaseStorage):
    """Implementation for supabase obs storage."""

    def __init__(self):
        super().__init__()
        if dify_config.SUPABASE_URL is None:
            raise ValueError("SUPABASE_URL is not set")
        if dify_config.SUPABASE_API_KEY is None:
            raise ValueError("SUPABASE_API_KEY is not set")
        if dify_config.SUPABASE_BUCKET_NAME is None:
            raise ValueError("SUPABASE_BUCKET_NAME is not set")

        self.bucket_name = dify_config.SUPABASE_BUCKET_NAME
        self.client = Client(supabase_url=dify_config.SUPABASE_URL, supabase_key=dify_config.SUPABASE_API_KEY)
        self.create_bucket(id=dify_config.SUPABASE_BUCKET_NAME, bucket_name=dify_config.SUPABASE_BUCKET_NAME)

    def create_bucket(self, id, bucket_name):
        if not self.bucket_exists():
            self.client.storage.create_bucket(id=id, name=bucket_name)

    def save(self, filename, data):
        self.client.storage.from_(self.bucket_name).upload(filename, data)

    def load_once(self, filename: str) -> bytes:
        content: bytes = self.client.storage.from_(self.bucket_name).download(filename)
        return content

    def load_stream(self, filename: str) -> Generator:
        result = self.client.storage.from_(self.bucket_name).download(filename)
        byte_stream = io.BytesIO(result)
        while chunk := byte_stream.read(4096):  # Read in chunks of 4KB
            yield chunk

    def download(self, filename, target_filepath):
        result = self.client.storage.from_(self.bucket_name).download(filename)
        Path(target_filepath).write_bytes(result)

    def exists(self, filename):
        result = self.client.storage.from_(self.bucket_name).list(path=filename)
        if len(result) > 0:
            return True
        return False

    def delete(self, filename: str):
        self.client.storage.from_(self.bucket_name).remove([filename])

    def bucket_exists(self):
        buckets = self.client.storage.list_buckets()
        return any(bucket.name == self.bucket_name for bucket in buckets)

```

### api/extensions/storage/opendal_storage.py
```py
import logging
import os
from collections.abc import Generator
from pathlib import Path

import opendal
from dotenv import dotenv_values
from opendal import Operator

from extensions.storage.base_storage import BaseStorage

logger = logging.getLogger(__name__)


def _get_opendal_kwargs(*, scheme: str, env_file_path: str = ".env", prefix: str = "OPENDAL_"):
    kwargs = {}
    config_prefix = prefix + scheme.upper() + "_"
    for key, value in os.environ.items():
        if key.startswith(config_prefix):
            kwargs[key[len(config_prefix) :].lower()] = value

    file_env_vars: dict = dotenv_values(env_file_path) or {}
    for key, value in file_env_vars.items():
        if key.startswith(config_prefix) and key[len(config_prefix) :].lower() not in kwargs and value:
            kwargs[key[len(config_prefix) :].lower()] = value

    return kwargs


class OpenDALStorage(BaseStorage):
    def __init__(self, scheme: str, **kwargs):
        kwargs = kwargs or _get_opendal_kwargs(scheme=scheme)

        if scheme == "fs":
            root = kwargs.setdefault("root", "storage")
            Path(root).mkdir(parents=True, exist_ok=True)

        retry_layer = opendal.layers.RetryLayer(max_times=3, factor=2.0, jitter=True)
        self.op = Operator(scheme=scheme, **kwargs).layer(retry_layer)
        logger.debug("opendal operator created with scheme %s", scheme)
        logger.debug("added retry layer to opendal operator")

    def save(self, filename: str, data: bytes):
        self.op.write(path=filename, bs=data)
        logger.debug("file %s saved", filename)

    def load_once(self, filename: str) -> bytes:
        if not self.exists(filename):
            raise FileNotFoundError("File not found")

        content: bytes = self.op.read(path=filename)
        logger.debug("file %s loaded", filename)
        return content

    def load_stream(self, filename: str) -> Generator:
        if not self.exists(filename):
            raise FileNotFoundError("File not found")

        batch_size = 4096
        with self.op.open(
            path=filename,
            mode="rb",
            chunck=batch_size,
        ) as file:
            while chunk := file.read(batch_size):
                yield chunk
        logger.debug("file %s loaded as stream", filename)

    def download(self, filename: str, target_filepath: str):
        if not self.exists(filename):
            raise FileNotFoundError("File not found")

        Path(target_filepath).write_bytes(self.op.read(path=filename))
        logger.debug("file %s downloaded to %s", filename, target_filepath)

    def exists(self, filename: str) -> bool:
        return self.op.exists(path=filename)

    def delete(self, filename: str):
        if self.exists(filename):
            self.op.delete(path=filename)
            logger.debug("file %s deleted", filename)
            return
        logger.debug("file %s not found, skip delete", filename)

    def scan(self, path: str, files: bool = True, directories: bool = False) -> list[str]:
        if not self.exists(path):
            raise FileNotFoundError("Path not found")

        # Use the new OpenDAL 0.46.0+ API with recursive listing
        lister = self.op.list(path, recursive=True)
        if files and directories:
            logger.debug("files and directories on %s scanned", path)
            return [entry.path for entry in lister]
        if files:
            logger.debug("files on %s scanned", path)
            return [entry.path for entry in lister if not entry.metadata.is_dir]
        elif directories:
            logger.debug("directories on %s scanned", path)
            return [entry.path for entry in lister if entry.metadata.is_dir]
        else:
            raise ValueError("At least one of files or directories must be True")

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-017.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
