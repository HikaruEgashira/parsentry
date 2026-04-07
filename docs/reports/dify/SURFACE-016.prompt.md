You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-016
- **Kind**: external_call
- **Identifier**: Website Crawl Provider (HTTP fetching of user-supplied URLs)
- **Description**: Fetches content from user-supplied URLs for knowledge base ingestion. Primary SSRF vector - must validate against internal network access, cloud metadata endpoints, and URL scheme restrictions
- **Locations**: api/core/datasource/website_crawl/website_crawl_provider.py, api/core/datasource/website_crawl/website_crawl_plugin.py

## Source Code

### api/core/datasource/website_crawl/website_crawl_provider.py
```py
from core.datasource.__base.datasource_provider import DatasourcePluginProviderController
from core.datasource.__base.datasource_runtime import DatasourceRuntime
from core.datasource.entities.datasource_entities import DatasourceProviderEntityWithPlugin, DatasourceProviderType
from core.datasource.website_crawl.website_crawl_plugin import WebsiteCrawlDatasourcePlugin


class WebsiteCrawlDatasourcePluginProviderController(DatasourcePluginProviderController):
    entity: DatasourceProviderEntityWithPlugin
    plugin_id: str
    plugin_unique_identifier: str

    def __init__(
        self,
        entity: DatasourceProviderEntityWithPlugin,
        plugin_id: str,
        plugin_unique_identifier: str,
        tenant_id: str,
    ) -> None:
        super().__init__(entity, tenant_id)
        self.plugin_id = plugin_id
        self.plugin_unique_identifier = plugin_unique_identifier

    @property
    def provider_type(self) -> DatasourceProviderType:
        """
        returns the type of the provider
        """
        return DatasourceProviderType.WEBSITE_CRAWL

    def get_datasource(self, datasource_name: str) -> WebsiteCrawlDatasourcePlugin:  # type: ignore
        """
        return datasource with given name
        """
        datasource_entity = next(
            (
                datasource_entity
                for datasource_entity in self.entity.datasources
                if datasource_entity.identity.name == datasource_name
            ),
            None,
        )

        if not datasource_entity:
            raise ValueError(f"Datasource with name {datasource_name} not found")

        return WebsiteCrawlDatasourcePlugin(
            entity=datasource_entity,
            runtime=DatasourceRuntime(tenant_id=self.tenant_id),
            tenant_id=self.tenant_id,
            icon=self.entity.identity.icon,
            plugin_unique_identifier=self.plugin_unique_identifier,
        )

```

### api/core/datasource/website_crawl/website_crawl_plugin.py
```py
from collections.abc import Generator, Mapping
from typing import Any

from core.datasource.__base.datasource_plugin import DatasourcePlugin
from core.datasource.__base.datasource_runtime import DatasourceRuntime
from core.datasource.entities.datasource_entities import (
    DatasourceEntity,
    DatasourceProviderType,
    WebsiteCrawlMessage,
)
from core.plugin.impl.datasource import PluginDatasourceManager


class WebsiteCrawlDatasourcePlugin(DatasourcePlugin):
    tenant_id: str
    plugin_unique_identifier: str
    entity: DatasourceEntity
    runtime: DatasourceRuntime

    def __init__(
        self,
        entity: DatasourceEntity,
        runtime: DatasourceRuntime,
        tenant_id: str,
        icon: str,
        plugin_unique_identifier: str,
    ) -> None:
        super().__init__(entity, runtime, icon)
        self.tenant_id = tenant_id
        self.plugin_unique_identifier = plugin_unique_identifier

    def get_website_crawl(
        self,
        user_id: str,
        datasource_parameters: Mapping[str, Any],
        provider_type: str,
    ) -> Generator[WebsiteCrawlMessage, None, None]:
        manager = PluginDatasourceManager()

        return manager.get_website_crawl(
            tenant_id=self.tenant_id,
            user_id=user_id,
            datasource_provider=self.entity.identity.provider,
            datasource_name=self.entity.identity.name,
            credentials=self.runtime.credentials,
            datasource_parameters=datasource_parameters,
            provider_type=provider_type,
        )

    def datasource_provider_type(self) -> str:
        return DatasourceProviderType.WEBSITE_CRAWL

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-016.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
