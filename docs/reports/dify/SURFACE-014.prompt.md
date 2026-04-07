You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-014
- **Kind**: endpoint
- **Identifier**: OAuth Callback and SSO (GET /oauth-callback)
- **Description**: OAuth redirect handler for third-party SSO. Risk of open redirect, authorization code injection, CSRF via missing state validation, and token fixation.
- **Locations**: web/app/oauth-callback/, web/app/signin/, api/controllers/console/auth/

## Repository Context

## Directory Structure
```
AGENTS.md
AUTHORS
CLAUDE.md
CONTRIBUTING.md
LICENSE
Makefile
README.md
api/ 
  AGENTS.md
  Dockerfile
  README.md
  app.py
  app_factory.py
  celery_entrypoint.py
  celery_healthcheck.py
  cnt_base.sh
  commands/ 
  configs/ 
    deploy/ 
    enterprise/ 
    extra/ 
    feature/ 
      hosted_service/ 
    middleware/ 
      cache/ 
      storage/ 
      vdb/ 
    observability/ 
      otel/ 
    packaging/ 
    remote_settings_sources/ 
      apollo/ 
      nacos/ 
  constants/ 
  context/ 
  contexts/ 
  controllers/ 
    common/ 
    console/ 
      app/ 
      auth/ 
      billing/ 
      datasets/ 
      explore/ 
      tag/ 
      workspace/ 
    files/ 
    inner_api/ 
      app/ 
      plugin/ 
      workspace/ 
    mcp/ 
    service_api/ 
      app/ 
      dataset/ 
      end_user/ 
      workspace/ 
    trigger/ 
    web/ 
  core/ 
    agent/ 
      output_parser/ 
      prompt/ 
      strategy/ 
    app/ 
      app_config/ 
      apps/ 
      entities/ 
      features/ 
      file_access/ 
      layers/ 
      llm/ 
      task_pipeline/ 
      workflow/ 
    base/ 
      tts/ 
    callback_handler/ 
    datasource/ 
      __base/ 
      entities/ 
      local_file/ 
      online_document/ 
      online_drive/ 
      utils/ 
      website_crawl/ 
    db/ 
    entities/ 
    errors/ 
    extension/ 
    external_data_tool/ 
      api/ 
    helper/ 
      code_executor/ 
    llm_generator/ 
      output_parser/ 
    logging/ 
    mcp/ 
      auth/ 
      client/ 
      server/ 
      session/ 
    memory/ 
    moderation/ 
      api/ 
      keywords/ 
      openai_moderation/ 
    ops/ 
      aliyun_trace/ 
      arize_phoenix_trace/ 
      entities/ 
      langfuse_trace/ 
      langsmith_trace/ 
      mlflow_trace/ 
      opik_trace/ 
      tencent_trace/ 
      weave_trace/ 
    plugin/ 
      backwards_invocation/ 
      endpoint/ 
      entities/ 
      impl/ 
      utils/ 
    prompt/ 
      entities/ 
      prompt_templates/ 
      utils/ 
    rag/ 
      cleaner/ 
      data_post_processor/ 
      datasource/ 
      docstore/ 
      embedding/ 
      entities/ 
      extractor/ 
      index_processor/ 
      models/ 
      pipeline/ 
      rerank/ 
      retrieval/ 
      splitter/ 
      summary_index/ 
    repositories/ 
    schemas/ 
      builtin/ 
    telemetry/ 
    tools/ 
      __base/ 
      builtin_tool/ 
      custom_tool/ 
      entities/ 
      mcp_tool/ 
      plugin_tool/ 
      utils/ 
      workflow_as_tool/ 
    trigger/ 
      debug/ 
      entities/ 
      utils/ 
    workflow/ 
      nodes/ 
  dify_app.py
  docker/ 
  enterprise/ 
    telemetry/ 
      entities/ 
  enums/ 
  events/ 
    event_handlers/ 
  extensions/ 
    logstore/ 
      repositories/ 
    otel/ 
      decorators/ 
      parser/ 
      semconv/ 
    storage/ 
      clickzetta_volume/ 
  factories/ 
    file_factory/ 
  fields/ 
  gunicorn.conf.py
  libs/ 
    broadcast_channel/ 
      redis/ 
  migrations/ 
    versions/ 
  models/ 
    utils/ 
  pyproject.toml
  pyrefly-local-excludes.txt
  pyrightconfig.json
  pytest.ini
  repositories/ 
    entities/ 
  schedule/ 
  services/ 
    auth/ 
      firecrawl/ 
      jina/ 
      watercrawl/ 
    document_indexing_proxy/ 
    enterprise/ 
    entities/ 
      external_knowledge_entities/ 
      knowledge_entities/ 
    errors/ 
    plugin/ 
    rag_pipeline/ 
      entity/ 
      pipeline_template/ 
      transform/ 
    recommend_app/ 
      buildin/ 
      database/ 
      remote/ 
    retention/ 
      conversation/ 
      workflow_run/ 
    tools/ 
    trigger/ 
    workflow/ 
  tasks/ 
    annotation/ 
    app_generate/ 
    rag_pipeline/ 
    workflow_cfs_scheduler/ 
  templates/ 
    without-brand/ 
  tests/ 
    fixtures/ 
      workflow/ 
    integration_tests/ 
      controllers/ 
      core/ 
      factories/ 
      libs/ 
      model_runtime/ 
      plugin/ 
      services/ 
      storage/ 
      tasks/ 
      tools/ 
      utils/ 
      vdb/ 
      workflow/ 
    test_containers_integration_tests/ 
      controllers/ 
      core/ 
      factories/ 
      helpers/ 
      libs/ 
      models/ 
      repositories/ 
      services/ 
      tasks/ 
      trigger/ 
      workflow/ 
    unit_tests/ 
      commands/ 
      configs/ 
      controllers/ 
      core/ 
      enterprise/ 
      events/ 
      extensions/ 
      factories/ 
      fields/ 
      libs/ 
      models/ 
      oss/ 
      repositories/ 
      services/ 
      tasks/ 
      tools/ 
      utils/ 
  uv.lock
codecov.yml
dev/ 
  basedpyright-check
  pyrefly-check-local
  pytest/ 
  reformat
  setup
  start-api
  start-beat
  start-docker-compose
  start-web
  start-worker
  sync-uv
  ty-check
  update-uv
docker/ 
  README.md
  certbot/ 
  couchbase-server/ 
  dify-env-sync.py
  dify-env-sync.sh
  docker-compose-template.yaml
  docker-compose.middleware.yaml
  docker-compose.png
  docker-compose.yaml
  elasticsearch/ 
  generate_docker_compose
  iris/ 
  middleware.env.example
  nginx/ 
    conf.d/ 
    ssl/ 
  pgvector/ 
  ssrf_proxy/ 
  startupscripts/ 
  tidb/ 
    config/ 
  volumes/ 
    myscale/ 
      config/ 
    oceanbase/ 
      init.d/ 
    opensearch/ 
    sandbox/ 
      conf/ 
docs/ 
  ar-SA/ 
  bn-BD/ 
  de-DE/ 
  es-ES/ 
  eu-ai-act-compliance.md
  fr-FR/ 
  hi-IN/ 
  it-IT/ 
  ja-JP/ 
  ko-KR/ 
  pt-BR/ 
  sl-SI/ 
  suggested-questions-configuration.md
  tlh/ 
  tr-TR/ 
  vi-VN/ 
  weaviate/ 
    WEAVIATE_MIGRATION_GUIDE/ 
  zh-CN/ 
  zh-TW/ 
e2e/ 
  AGENTS.md
  README.md
  cucumber.config.ts
  features/ 
    apps/ 
    smoke/ 
    step-definitions/ 
      apps/ 
      common/ 
      smoke/ 
    support/ 
  fixtures/ 
  package.json
  scripts/ 
  support/ 
  test-env.ts
  tsconfig.json
  vite.config.ts
images/ 
  GitHub_README_if.png
  describe.png
  models.png
package.json
packages/ 
  iconify-collections/ 
    assets/ 
      public/ 
      vender/ 
    custom-public/ 
    custom-vender/ 
    scripts/ 
pnpm-lock.yaml
pnpm-workspace.yaml
scripts/ 
  stress-test/ 
    common/ 
    setup/ 
      dsl/ 
sdks/ 
  README.md
  nodejs-client/ 
    scripts/ 
    src/ 
      client/ 
      errors/ 
      http/ 
      internal/ 
      types/ 
    tests/ 
  php-client/ 
vite.config.ts
web/ 
  AGENTS.md
  CLAUDE.md
  Dockerfile
  Dockerfile.dockerignore
  README.md
  __mocks__/ 
    @tanstack/ 
  __tests__/ 
    apps/ 
    billing/ 
    datasets/ 
    develop/ 
    explore/ 
    goto-anything/ 
    plugins/ 
    rag-pipeline/ 
    share/ 
    tools/ 
  app/ 
    (commonLayout)/ 
      app/ 
      apps/ 
      datasets/ 
      education-apply/ 
      explore/ 
      plugins/ 
      tools/ 
    (humanInputLayout)/ 
      form/ 
    (shareLayout)/ 
      chat/ 
      chatbot/ 
      completion/ 
      components/ 
      webapp-reset-password/ 
      webapp-signin/ 
      workflow/ 
    account/ 
      (commonLayout)/ 
      oauth/ 
    activate/ 
    components/ 
      app/ 
      app-sidebar/ 
      apps/ 
      base/ 
      billing/ 
      custom/ 
      datasets/ 
      develop/ 
      devtools/ 
      explore/ 
      goto-anything/ 
      header/ 
      plugins/ 
      provider/ 
      rag-pipeline/ 
      share/ 
      signin/ 
      tools/ 
      workflow/ 
      workflow-app/ 
    education-apply/ 
    forgot-password/ 
    init/ 
    install/ 
    oauth-callback/ 
    reset-password/ 
      check-code/ 
      set-password/ 
    signin/ 
      assets/ 
      check-code/ 
      components/ 
      invite-settings/ 
      utils/ 
    signup/ 
      check-code/ 
      components/ 
      set-password/ 
    styles/ 
  assets/ 
  bin/ 
  config/ 
  constants/ 
  context/ 
    hooks/ 
  contract/ 
    console/ 
  docker/ 
  docs/ 
  env.ts
  eslint-suppressions.json
  eslint.config.mjs
  eslint.constants.mjs
  global.d.ts
  hooks/ 
  i18n/ 
    ar-TN/ 
    de-DE/ 
    en-US/ 
    es-ES/ 
    fa-IR/ 
    fr-FR/ 
    hi-IN/ 
    id-ID/ 
    it-IT/ 
    ja-JP/ 
    ko-KR/ 
    nl-NL/ 
    pl-PL/ 
    pt-BR/ 
    ro-RO/ 
    ru-RU/ 
    sl-SI/ 
    th-TH/ 
    tr-TR/ 
    uk-UA/ 
    vi-VN/ 
    zh-Hans/ 
    zh-Hant/ 
  i18n-config/ 
  instrumentation-client.ts
  knip.config.ts
  models/ 
  next/ 
  next.config.ts
  package.json
  plugins/ 
    dev-proxy/ 
    eslint/ 
      rules/ 
    vite/ 
  postcss.config.js
  proxy.ts
  public/ 
    education/ 
    in-site-message/ 
    logo/ 
    screenshots/ 
      dark/ 
      light/ 
    vs/ 
      base/ 
      basic-languages/ 
      editor/ 
      language/ 
  scripts/ 
  service/ 
    knowledge/ 
  tailwind-common-config.ts
  tailwind.config.ts
  test/ 
  themes/ 
  tsconfig.json
  tsslint.config.ts
  types/ 
  typography.js
  utils/ 
  vite.config.ts
  vitest.setup.ts

```

## Languages
- TypeScript: 5508 files
- Python: 2785 files
- JavaScript: 122 files
- Yaml: 95 files
- Bash: 20 files
- Php: 1 files

## Dependencies
### package.json
```
{
  "name": "dify",
  "private": true,
  "scripts": {
    "prepare": "vp config"
  },
  "devDependencies": {
    "vite-plus": "catalog:"
  },
  "engines": {
    "node": "^22.22.1"
  },
  "packageManager": "pnpm@10.33.0"
}

```

## Entry Points
- sdks/nodejs-client/src/index.ts
- web/next/index.ts
- web/types/app.ts
- web/app/components/tools/utils/index.ts
- web/app/components/plugins/plugin-detail-panel/tool-selector/components/index.ts
- web/app/components/plugins/plugin-detail-panel/tool-selector/hooks/index.ts
- web/app/components/plugins/plugin-detail-panel/detail-header/components/index.ts
- web/app/components/plugins/plugin-detail-panel/detail-header/hooks/index.ts
- web/app/components/goto-anything/components/index.ts
- web/app/components/goto-anything/hooks/index.ts
- web/app/components/goto-anything/actions/commands/index.ts
- web/app/components/goto-anything/actions/index.ts
- web/app/components/workflow-app/hooks/index.ts
- web/app/components/datasets/documents/create-from-pipeline/data-source/store/index.ts
- web/app/components/datasets/documents/create-from-pipeline/steps/index.ts
- web/app/components/datasets/documents/create-from-pipeline/hooks/index.ts
- web/app/components/datasets/documents/components/document-list/components/index.ts
- web/app/components/datasets/documents/components/document-list/hooks/index.ts
- web/app/components/datasets/documents/detail/completed/components/index.ts
- web/app/components/datasets/documents/detail/completed/hooks/index.ts
- web/app/components/datasets/documents/detail/embedding/components/index.ts
- web/app/components/datasets/documents/detail/embedding/hooks/index.ts
- web/app/components/datasets/create/step-one/components/index.ts
- web/app/components/datasets/create/step-one/hooks/index.ts
- web/app/components/datasets/create/step-two/components/index.ts
- web/app/components/datasets/create/step-two/hooks/index.ts
- web/app/components/rag-pipeline/utils/index.ts
- web/app/components/rag-pipeline/hooks/index.ts
- web/app/components/rag-pipeline/store/index.ts
- web/app/components/workflow/hooks-store/index.ts
- web/app/components/workflow/note-node/note-editor/theme/index.ts
- web/app/components/workflow/utils/index.ts
- web/app/components/workflow/hooks/use-workflow-run-event/index.ts
- web/app/components/workflow/hooks/index.ts
- web/app/components/workflow/run/utils/format-log/parallel/index.ts
- web/app/components/workflow/run/utils/format-log/retry/index.ts
- web/app/components/workflow/run/utils/format-log/human-input/index.ts
- web/app/components/workflow/run/utils/format-log/agent/index.ts
- web/app/components/workflow/run/utils/format-log/index.ts
- web/app/components/workflow/run/utils/format-log/iteration/index.ts
- web/app/components/workflow/run/utils/format-log/loop/index.ts
- web/app/components/workflow/store/workflow/index.ts
- web/app/components/workflow/store/index.ts
- web/app/components/header/account-setting/model-provider-page/model-auth/hooks/index.ts
- web/app/components/header/account-setting/data-source-page-new/hooks/index.ts
- web/app/components/base/form/utils/secret-input/index.ts
- web/app/components/base/form/hooks/index.ts
- web/app/components/base/radio/context/index.ts
- web/app/components/base/amplitude/index.ts
- web/app/components/base/markdown-blocks/index.ts
- web/app/components/base/icons/src/public/tracing/index.ts
- web/app/components/base/icons/src/public/llm/index.ts
- web/app/components/base/icons/src/public/education/index.ts
- web/app/components/base/icons/src/public/other/index.ts
- web/app/components/base/icons/src/public/common/index.ts
- web/app/components/base/icons/src/public/knowledge/dataset-card/index.ts
- web/app/components/base/icons/src/public/knowledge/index.ts
- web/app/components/base/icons/src/public/knowledge/online-drive/index.ts
- web/app/components/base/icons/src/public/avatar/index.ts
- web/app/components/base/icons/src/public/files/index.ts
- web/app/components/base/icons/src/public/thought/index.ts
- web/app/components/base/icons/src/public/billing/index.ts
- web/app/components/base/icons/src/vender/pipeline/index.ts
- web/app/components/base/icons/src/vender/features/index.ts
- web/app/components/base/icons/src/vender/other/index.ts
- web/app/components/base/icons/src/vender/plugin/index.ts
- web/app/components/base/icons/src/vender/solid/mediaAndDevices/index.ts
- web/app/components/base/icons/src/vender/solid/security/index.ts
- web/app/components/base/icons/src/vender/solid/general/index.ts
- web/app/components/base/icons/src/vender/solid/development/index.ts
- web/app/components/base/icons/src/vender/solid/education/index.ts
- web/app/components/base/icons/src/vender/solid/shapes/index.ts
- web/app/components/base/icons/src/vender/solid/users/index.ts
- web/app/components/base/icons/src/vender/solid/files/index.ts
- web/app/components/base/icons/src/vender/solid/arrows/index.ts
- web/app/components/base/icons/src/vender/solid/communication/index.ts
- web/app/components/base/icons/src/vender/solid/editor/index.ts
- web/app/components/base/icons/src/vender/solid/FinanceAndECommerce/index.ts
- web/app/components/base/icons/src/vender/solid/alertsAndFeedback/index.ts
- web/app/components/base/icons/src/vender/system/index.ts
- web/app/components/base/icons/src/vender/knowledge/index.ts
- web/app/components/base/icons/src/vender/line/mediaAndDevices/index.ts
- web/app/components/base/icons/src/vender/line/images/index.ts
- web/app/components/base/icons/src/vender/line/general/index.ts
- web/app/components/base/icons/src/vender/line/development/index.ts
- web/app/components/base/icons/src/vender/line/layout/index.ts
- web/app/components/base/icons/src/vender/line/education/index.ts
- web/app/components/base/icons/src/vender/line/others/index.ts
- web/app/components/base/icons/src/vender/line/time/index.ts
- web/app/components/base/icons/src/vender/line/files/index.ts
- web/app/components/base/icons/src/vender/line/arrows/index.ts
- web/app/components/base/icons/src/vender/line/communication/index.ts
- web/app/components/base/icons/src/vender/line/editor/index.ts
- web/app/components/base/icons/src/vender/line/financeAndECommerce/index.ts
- web/app/components/base/icons/src/vender/line/alertsAndFeedback/index.ts
- web/app/components/base/icons/src/vender/workflow/index.ts
- web/app/components/base/file-uploader/index.ts
- web/app/components/billing/utils/index.ts
- web/config/index.ts
- web/plugins/eslint/index.js
- web/plugins/dev-proxy/server.ts
- web/utils/index.ts
- web/models/app.ts
- web/i18n-config/index.ts
- web/i18n-config/server.ts
- packages/iconify-collections/custom-vender/index.js
- packages/iconify-collections/custom-public/index.js
- api/core/plugin/backwards_invocation/app.py
- api/app.py
- api/controllers/web/app.py
- api/controllers/service_api/app/app.py
- api/controllers/console/app/app.py
- api/services/errors/app.py

Total source files: 8531


## Source Code

### web/app/oauth-callback/page.tsx
```tsx
'use client'
import { useOAuthCallback } from '@/hooks/use-oauth'

const OAuthCallback = () => {
  useOAuthCallback()

  return <div />
}

export default OAuthCallback

```

### web/app/signin/normal-form.tsx
```tsx
import { RiContractLine, RiDoorLockLine, RiErrorWarningFill } from '@remixicon/react'
import * as React from 'react'
import { useCallback, useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from '@/app/components/base/ui/toast'
import { IS_CE_EDITION } from '@/config'
import { useGlobalPublicStore } from '@/context/global-public-context'
import Link from '@/next/link'
import { useRouter, useSearchParams } from '@/next/navigation'
import { invitationCheck } from '@/service/common'
import { useIsLogin } from '@/service/use-common'
import { LicenseStatus } from '@/types/feature'
import { cn } from '@/utils/classnames'
import Loading from '../components/base/loading'
import MailAndCodeAuth from './components/mail-and-code-auth'
import MailAndPasswordAuth from './components/mail-and-password-auth'
import SocialAuth from './components/social-auth'
import SSOAuth from './components/sso-auth'
import Split from './split'
import { resolvePostLoginRedirect } from './utils/post-login-redirect'

const NormalForm = () => {
  const { t } = useTranslation()
  const router = useRouter()
  const searchParams = useSearchParams()
  const { isLoading: isCheckLoading, data: loginData } = useIsLogin()
  const isLoggedIn = loginData?.logged_in
  const message = decodeURIComponent(searchParams.get('message') || '')
  const invite_token = decodeURIComponent(searchParams.get('invite_token') || '')
  const [isInitCheckLoading, setInitCheckLoading] = useState(true)
  const [isRedirecting, setIsRedirecting] = useState(false)
  const isLoading = isCheckLoading || isInitCheckLoading || isRedirecting
  const { systemFeatures } = useGlobalPublicStore()
  const [authType, updateAuthType] = useState<'code' | 'password'>('password')
  const [showORLine, setShowORLine] = useState(false)
  const [allMethodsAreDisabled, setAllMethodsAreDisabled] = useState(false)
  const [workspaceName, setWorkSpaceName] = useState('')

  const isInviteLink = Boolean(invite_token && invite_token !== 'null')

  const init = useCallback(async () => {
    try {
      if (isLoggedIn) {
        setIsRedirecting(true)
        const redirectUrl = resolvePostLoginRedirect()
        router.replace(redirectUrl || '/apps')
        return
      }

      if (message) {
        toast.error(message)
      }
      setAllMethodsAreDisabled(!systemFeatures.enable_social_oauth_login && !systemFeatures.enable_email_code_login && !systemFeatures.enable_email_password_login && !systemFeatures.sso_enforced_for_signin)
      setShowORLine((systemFeatures.enable_social_oauth_login || systemFeatures.sso_enforced_for_signin) && (systemFeatures.enable_email_code_login || systemFeatures.enable_email_password_login))
      updateAuthType(systemFeatures.enable_email_password_login ? 'password' : 'code')
      if (isInviteLink) {
        const checkRes = await invitationCheck({
          url: '/activate/check',
          params: {
            token: invite_token,
          },
        })
        setWorkSpaceName(checkRes?.data?.workspace_name || '')
      }
    }
    catch (error) {
      console.error(error)
      setAllMethodsAreDisabled(true)
    }
    finally { setInitCheckLoading(false) }
  }, [isLoggedIn, message, router, invite_token, isInviteLink, systemFeatures])
  useEffect(() => {
    init()
  }, [init])
  if (isLoading) {
    return (
      <div className={
        cn(
          'flex w-full grow flex-col items-center justify-center',
          'px-6',
          'md:px-[108px]',
        )
      }
      >
        <Loading type="area" />
      </div>
    )
  }
  if (systemFeatures.license?.status === LicenseStatus.LOST) {
    return (
      <div className="mx-auto mt-8 w-full">
        <div className="relative">
          <div className="rounded-lg bg-linear-to-r from-workflow-workflow-progress-bg-1 to-workflow-workflow-progress-bg-2 p-4">
            <div className="shadows-shadow-lg relative mb-2 flex h-10 w-10 items-center justify-center rounded-xl bg-components-card-bg shadow">
              <RiContractLine className="h-5 w-5" />
              <RiErrorWarningFill className="absolute -right-1 -top-1 h-4 w-4 text-text-warning-secondary" />
            </div>
            <p className="text-text-primary system-sm-medium">{t('licenseLost', { ns: 'login' })}</p>
            <p className="mt-1 text-text-tertiary system-xs-regular">{t('licenseLostTip', { ns: 'login' })}</p>
          </div>
        </div>
      </div>
    )
  }
  if (systemFeatures.license?.status === LicenseStatus.EXPIRED) {
    return (
      <div className="mx-auto mt-8 w-full">
        <div className="relative">
          <div className="rounded-lg bg-linear-to-r from-workflow-workflow-progress-bg-1 to-workflow-workflow-progress-bg-2 p-4">
            <div className="shadows-shadow-lg relative mb-2 flex h-10 w-10 items-center justify-center rounded-xl bg-components-card-bg shadow">
              <RiContractLine className="h-5 w-5" />
              <RiErrorWarningFill className="absolute -right-1 -top-1 h-4 w-4 text-text-warning-secondary" />
            </div>
            <p className="text-text-primary system-sm-medium">{t('licenseExpired', { ns: 'login' })}</p>
            <p className="mt-1 text-text-tertiary system-xs-regular">{t('licenseExpiredTip', { ns: 'login' })}</p>
          </div>
        </div>
      </div>
    )
  }
  if (systemFeatures.license?.status === LicenseStatus.INACTIVE) {
    return (
      <div className="mx-auto mt-8 w-full">
        <div className="relative">
          <div className="rounded-lg bg-linear-to-r from-workflow-workflow-progress-bg-1 to-workflow-workflow-progress-bg-2 p-4">
            <div className="shadows-shadow-lg relative mb-2 flex h-10 w-10 items-center justify-center rounded-xl bg-components-card-bg shadow">
              <RiContractLine className="h-5 w-5" />
              <RiErrorWarningFill className="absolute -right-1 -top-1 h-4 w-4 text-text-warning-secondary" />
            </div>
            <p className="text-text-primary system-sm-medium">{t('licenseInactive', { ns: 'login' })}</p>
            <p className="mt-1 text-text-tertiary system-xs-regular">{t('licenseInactiveTip', { ns: 'login' })}</p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <>
      <div className="mx-auto mt-8 w-full">
        {isInviteLink
          ? (
              <div className="mx-auto w-full">
                <h2 className="text-text-primary title-4xl-semi-bold">
                  {t('join', { ns: 'login' })}
                  {workspaceName}
                </h2>
                {!systemFeatures.branding.enabled && (
                  <p className="mt-2 text-text-tertiary body-md-regular">
                    {t('joinTipStart', { ns: 'login' })}
                    {workspaceName}
                    {t('joinTipEnd', { ns: 'login' })}
                  </p>
                )}
              </div>
            )
          : (
              <div className="mx-auto w-full">
                <h2 className="text-text-primary title-4xl-semi-bold">{systemFeatures.branding.enabled ? t('pageTitleForE', { ns: 'login' }) : t('pageTitle', { ns: 'login' })}</h2>
                <p className="mt-2 text-text-tertiary body-md-regular">{t('welcome', { ns: 'login' })}</p>
              </div>
            )}
        <div className="relative">
          <div className="mt-6 flex flex-col gap-3">
            {systemFeatures.enable_social_oauth_login && <SocialAuth />}
            {systemFeatures.sso_enforced_for_signin && (
              <div className="w-full">
                <SSOAuth protocol={systemFeatures.sso_enforced_for_signin_protocol} />
              </div>
            )}
          </div>

          {showORLine && (
            <div className="relative mt-6">
              <div className="flex items-center">
                <div className="h-px flex-1 bg-linear-to-r from-background-gradient-mask-transparent to-divider-regular"></div>
                <span className="px-3 text-text-tertiary system-xs-medium-uppercase">{t('or', { ns: 'login' })}</span>
                <div className="h-px flex-1 bg-linear-to-l from-background-gradient-mask-transparent to-divider-regular"></div>
              </div>
            </div>
          )}
          {
            (systemFeatures.enable_email_code_login || systemFeatures.enable_email_password_login) && (
              <>
                {systemFeatures.enable_email_code_login && authType === 'code' && (
                  <>
                    <MailAndCodeAuth isInvite={isInviteLink} />
                    {systemFeatures.enable_email_password_login && (
                      <div className="cursor-pointer py-1 text-center" onClick={() => { updateAuthType('password') }}>
                        <span className="text-components-button-secondary-accent-text system-xs-medium">{t('usePassword', { ns: 'login' })}</span>
                      </div>
                    )}
                  </>
                )}
                {systemFeatures.enable_email_password_login && authType === 'password' && (
                  <>
                    <MailAndPasswordAuth isInvite={isInviteLink} isEmailSetup={systemFeatures.is_email_setup} allowRegistration={systemFeatures.is_allow_register} />
                    {systemFeatures.enable_email_code_login && (
                      <div className="cursor-pointer py-1 text-center" onClick={() => { updateAuthType('code') }}>
                        <span className="text-components-button-secondary-accent-text system-xs-medium">{t('useVerificationCode', { ns: 'login' })}</span>
                      </div>
                    )}
                  </>
                )}
                <Split className="mb-5 mt-4" />
              </>
            )
          }

          {systemFeatures.is_allow_register && authType === 'password' && (
            <div className="mb-3 text-[13px] font-medium leading-4 text-text-secondary">
              <span>{t('signup.noAccount', { ns: 'login' })}</span>
              <Link
                className="text-text-accent"
                href="/signup"
              >
                {t('signup.signUp', { ns: 'login' })}
              </Link>
            </div>
          )}
          {allMethodsAreDisabled && (
            <>
              <div className="rounded-lg bg-linear-to-r from-workflow-workflow-progress-bg-1 to-workflow-workflow-progress-bg-2 p-4">
                <div className="shadows-shadow-lg mb-2 flex h-10 w-10 items-center justify-center rounded-xl bg-components-card-bg shadow">
                  <RiDoorLockLine className="h-5 w-5" />
                </div>
                <p className="text-text-primary system-sm-medium">{t('noLoginMethod', { ns: 'login' })}</p>
                <p className="mt-1 text-text-tertiary system-xs-regular">{t('noLoginMethodTip', { ns: 'login' })}</p>
              </div>
              <div className="relative my-2 py-2">
                <div className="absolute inset-0 flex items-center" aria-hidden="true">
                  <div className="h-px w-full bg-linear-to-r from-background-gradient-mask-transparent via-divider-regular to-background-gradient-mask-transparent"></div>
                </div>
              </div>
            </>
          )}
          {!systemFeatures.branding.enabled && (
            <>
              <div className="mt-2 block w-full text-text-tertiary system-xs-regular">
                {t('tosDesc', { ns: 'login' })}
              &nbsp;
                <Link
                  className="text-text-secondary system-xs-medium hover:underline"
                  target="_blank"
                  rel="noopener noreferrer"
                  href="https://dify.ai/terms"
                >
                  {t('tos', { ns: 'login' })}
                </Link>
              &nbsp;&&nbsp;
                <Link
                  className="text-text-secondary system-xs-medium hover:underline"
                  target="_blank"
                  rel="noopener noreferrer"
                  href="https://dify.ai/privacy"
                >
                  {t('pp', { ns: 'login' })}
                </Link>
              </div>
              {IS_CE_EDITION && (
                <div className="w-hull mt-2 block text-text-tertiary system-xs-regular">
                  {t('goToInit', { ns: 'login' })}
              &nbsp;
                  <Link
                    className="text-text-secondary system-xs-medium hover:underline"
                    href="/install"
                  >
                    {t('setAdminAccount', { ns: 'login' })}
                  </Link>
                </div>
              )}
            </>
          )}
        </div>
      </div>
    </>
  )
}

export default NormalForm

```

### web/app/signin/invite-settings/page.tsx
```tsx
'use client'
import type { Locale } from '@/i18n-config'
import { RiAccountCircleLine } from '@remixicon/react'
import { noop } from 'es-toolkit/function'
import { useCallback, useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import Loading from '@/app/components/base/loading'
import { SimpleSelect } from '@/app/components/base/select'
import { toast } from '@/app/components/base/ui/toast'
import { LICENSE_LINK } from '@/constants/link'
import { useGlobalPublicStore } from '@/context/global-public-context'
import { setLocaleOnClient } from '@/i18n-config'
import { languages, LanguagesSupported } from '@/i18n-config/language'
import Link from '@/next/link'
import { useRouter, useSearchParams } from '@/next/navigation'
import { activateMember } from '@/service/common'
import { useInvitationCheck } from '@/service/use-common'
import { timezones } from '@/utils/timezone'
import { resolvePostLoginRedirect } from '../utils/post-login-redirect'

export default function InviteSettingsPage() {
  const { t } = useTranslation()
  const systemFeatures = useGlobalPublicStore(s => s.systemFeatures)
  const router = useRouter()
  const searchParams = useSearchParams()
  const token = decodeURIComponent(searchParams.get('invite_token') as string)
  const [name, setName] = useState('')
  const [language, setLanguage] = useState(LanguagesSupported[0])
  const [timezone, setTimezone] = useState(() => Intl.DateTimeFormat().resolvedOptions().timeZone || 'America/Los_Angeles')

  const checkParams = {
    url: '/activate/check',
    params: {
      token,
    },
  }
  const { data: checkRes, refetch: recheck } = useInvitationCheck(checkParams.params, !!token)

  const handleActivate = useCallback(async () => {
    try {
      if (!name) {
        toast.error(t('enterYourName', { ns: 'login' }))
        return
      }
      const res = await activateMember({
        url: '/activate',
        body: {
          token,
          name,
          interface_language: language,
          timezone,
        },
      })
      if (res.result === 'success') {
        // Tokens are now stored in cookies by the backend
        await setLocaleOnClient(language, false)
        const redirectUrl = resolvePostLoginRedirect()
        router.replace(redirectUrl || '/apps')
      }
    }
    catch {
      recheck()
    }
  }, [language, name, recheck, timezone, token, router, t])

  if (!checkRes)
    return <Loading />
  if (!checkRes.is_valid) {
    return (
      <div className="flex flex-col md:w-[400px]">
        <div className="mx-auto w-full">
          <div className="mb-3 flex h-14 w-14 items-center justify-center rounded-2xl border border-components-panel-border-subtle text-2xl font-bold shadow-lg">🤷‍♂️</div>
          <h2 className="text-text-primary title-4xl-semi-bold">{t('invalid', { ns: 'login' })}</h2>
        </div>
        <div className="mx-auto mt-6 w-full">
          <Button variant="primary" className="w-full text-sm!">
            <a href="https://dify.ai">{t('explore', { ns: 'login' })}</a>
          </Button>
        </div>
      </div>
    )
  }

  return (
    <div className="flex flex-col gap-3">
      <div className="inline-flex h-14 w-14 items-center justify-center rounded-2xl border border-components-panel-border-subtle bg-background-default-dodge shadow-lg">
        <RiAccountCircleLine className="h-6 w-6 text-2xl text-text-accent-light-mode-only" />
      </div>
      <div className="pb-4 pt-2">
        <h2 className="text-text-primary title-4xl-semi-bold">{t('setYourAccount', { ns: 'login' })}</h2>
      </div>
      <form onSubmit={noop}>
        <div className="mb-5">
          <label htmlFor="name" className="my-2 text-text-secondary system-md-semibold">
            {t('name', { ns: 'login' })}
          </label>
          <div className="mt-1">
            <Input
              id="name"
              type="text"
              value={name}
              onChange={e => setName(e.target.value)}
              placeholder={t('namePlaceholder', { ns: 'login' }) || ''}
              onKeyDown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault()
                  e.stopPropagation()
                  handleActivate()
                }
              }}
            />
          </div>
        </div>
        <div className="mb-5">
          <label htmlFor="name" className="my-2 text-text-secondary system-md-semibold">
            {t('interfaceLanguage', { ns: 'login' })}
          </label>
          <div className="mt-1">
            <SimpleSelect
              defaultValue={LanguagesSupported[0]}
              items={languages.filter(item => item.supported)}
              onSelect={(item) => {
                setLanguage(item.value as Locale)
              }}
            />
          </div>
        </div>
        {/* timezone */}
        <div className="mb-5">
          <label htmlFor="timezone" className="text-text-secondary system-md-semibold">
            {t('timezone', { ns: 'login' })}
          </label>
          <div className="mt-1">
            <SimpleSelect
              defaultValue={timezone}
              items={timezones}
              onSelect={(item) => {
                setTimezone(item.value as string)
              }}
            />
          </div>
        </div>
        <div>
          <Button
            variant="primary"
            className="w-full"
            onClick={handleActivate}
          >
            {`${t('join', { ns: 'login' })} ${checkRes?.data?.workspace_name}`}
          </Button>
        </div>
      </form>
      {!systemFeatures.branding.enabled && (
        <div className="mt-2 block w-full text-text-tertiary system-xs-regular">
          {t('license.tip', { ns: 'login' })}
      &nbsp;
          <Link
            className="text-text-accent-secondary system-xs-medium"
            target="_blank"
            rel="noopener noreferrer"
            href={LICENSE_LINK}
          >
            {t('license.link', { ns: 'login' })}
          </Link>
        </div>
      )}
    </div>
  )
}

```

### web/app/signin/check-code/page.tsx
```tsx
'use client'
import type { FormEvent } from 'react'
import { RiArrowLeftLine, RiMailSendFill } from '@remixicon/react'
import { useEffect, useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { trackEvent } from '@/app/components/base/amplitude'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import Countdown from '@/app/components/signin/countdown'
import { useLocale } from '@/context/i18n'

import { useRouter, useSearchParams } from '@/next/navigation'
import { emailLoginWithCode, sendEMailLoginCode } from '@/service/common'
import { encryptVerificationCode } from '@/utils/encryption'
import { resolvePostLoginRedirect } from '../utils/post-login-redirect'

export default function CheckCode() {
  const { t, i18n } = useTranslation()
  const router = useRouter()
  const searchParams = useSearchParams()
  const email = decodeURIComponent(searchParams.get('email') as string)
  const token = decodeURIComponent(searchParams.get('token') as string)
  const invite_token = decodeURIComponent(searchParams.get('invite_token') || '')
  const language = i18n.language
  const [code, setVerifyCode] = useState('')
  const [loading, setIsLoading] = useState(false)
  const locale = useLocale()
  const codeInputRef = useRef<HTMLInputElement>(null)

  const verify = async () => {
    try {
      if (!code.trim()) {
        toast.error(t('checkCode.emptyCode', { ns: 'login' }))
        return
      }
      if (!/\d{6}/.test(code)) {
        toast.error(t('checkCode.invalidCode', { ns: 'login' }))
        return
      }
      setIsLoading(true)
      const ret = await emailLoginWithCode({ email, code: encryptVerificationCode(code), token, language })
      if (ret.result === 'success') {
        // Track login success event
        trackEvent('user_login_success', {
          method: 'email_code',
          is_invite: !!invite_token,
        })

        if (invite_token) {
          router.replace(`/signin/invite-settings?${searchParams.toString()}`)
        }
        else {
          const redirectUrl = resolvePostLoginRedirect()
          router.replace(redirectUrl || '/apps')
        }
      }
    }
    catch (error) { console.error(error) }
    finally {
      setIsLoading(false)
    }
  }

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    verify()
  }

  useEffect(() => {
    codeInputRef.current?.focus()
  }, [])

  const resendCode = async () => {
    try {
      const ret = await sendEMailLoginCode(email, locale)
      if (ret.result === 'success') {
        const params = new URLSearchParams(searchParams)
        params.set('token', encodeURIComponent(ret.data))
        router.replace(`/signin/check-code?${params.toString()}`)
      }
    }
    catch (error) { console.error(error) }
  }

  return (
    <div className="flex flex-col gap-3">
      <div className="inline-flex h-14 w-14 items-center justify-center rounded-2xl border border-components-panel-border-subtle bg-background-default-dodge shadow-lg">
        <RiMailSendFill className="h-6 w-6 text-2xl text-text-accent-light-mode-only" />
      </div>
      <div className="pb-4 pt-2">
        <h2 className="text-text-primary title-4xl-semi-bold">{t('checkCode.checkYourEmail', { ns: 'login' })}</h2>
        <p className="mt-2 text-text-secondary body-md-regular">
          <span>
            {t('checkCode.tipsPrefix', { ns: 'login' })}
            <strong>{email}</strong>
          </span>
          <br />
          {t('checkCode.validTime', { ns: 'login' })}
        </p>
      </div>

      <form onSubmit={handleSubmit}>
        <label htmlFor="code" className="mb-1 text-text-secondary system-md-semibold">{t('checkCode.verificationCode', { ns: 'login' })}</label>
        <Input
          ref={codeInputRef}
          id="code"
          value={code}
          onChange={e => setVerifyCode(e.target.value)}
          maxLength={6}
          className="mt-1"
          placeholder={t('checkCode.verificationCodePlaceholder', { ns: 'login' }) as string}
        />
        <Button type="submit" loading={loading} disabled={loading} className="my-3 w-full" variant="primary">{t('checkCode.verify', { ns: 'login' })}</Button>
        <Countdown onResend={resendCode} />
      </form>
      <div className="py-2">
        <div className="h-px bg-linear-to-r from-background-gradient-mask-transparent via-divider-regular to-background-gradient-mask-transparent"></div>
      </div>
      <div onClick={() => router.back()} className="flex h-9 cursor-pointer items-center justify-center text-text-tertiary">
        <div className="inline-block rounded-full bg-background-default-dimmed p-1">
          <RiArrowLeftLine size={12} />
        </div>
        <span className="ml-2 system-xs-regular">{t('back', { ns: 'login' })}</span>
      </div>
    </div>
  )
}

```

### web/app/signin/utils/post-login-redirect.ts
```ts
let postLoginRedirect: string | null = null

export const setPostLoginRedirect = (value: string | null) => {
  postLoginRedirect = value
}

export const resolvePostLoginRedirect = () => {
  if (postLoginRedirect) {
    const redirectUrl = postLoginRedirect
    postLoginRedirect = null
    return redirectUrl
  }

  return null
}

```

### web/app/signin/components/mail-and-password-auth.tsx
```tsx
import type { ResponseError } from '@/service/fetch'
import { noop } from 'es-toolkit/function'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { trackEvent } from '@/app/components/base/amplitude'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import { emailRegex } from '@/config'
import { useLocale } from '@/context/i18n'
import Link from '@/next/link'
import { useRouter, useSearchParams } from '@/next/navigation'
import { login } from '@/service/common'
import { setWebAppAccessToken } from '@/service/webapp-auth'
import { encryptPassword } from '@/utils/encryption'
import { resolvePostLoginRedirect } from '../utils/post-login-redirect'

type MailAndPasswordAuthProps = {
  isInvite: boolean
  isEmailSetup: boolean
  allowRegistration: boolean
}

export default function MailAndPasswordAuth({ isInvite, isEmailSetup, allowRegistration: _allowRegistration }: MailAndPasswordAuthProps) {
  const { t } = useTranslation()
  const locale = useLocale()
  const router = useRouter()
  const searchParams = useSearchParams()
  const [showPassword, setShowPassword] = useState(false)
  const emailFromLink = decodeURIComponent(searchParams.get('email') || '')
  const [email, setEmail] = useState(emailFromLink)
  const [password, setPassword] = useState('')

  const [isLoading, setIsLoading] = useState(false)

  const handleEmailPasswordLogin = async () => {
    if (!email) {
      toast.error(t('error.emailEmpty', { ns: 'login' }))
      return
    }
    if (!emailRegex.test(email)) {
      toast.error(t('error.emailInValid', { ns: 'login' }))
      return
    }
    if (!password?.trim()) {
      toast.error(t('error.passwordEmpty', { ns: 'login' }))
      return
    }

    try {
      setIsLoading(true)
      const loginData: Record<string, any> = {
        email,
        password: encryptPassword(password),
        language: locale,
        remember_me: true,
      }
      if (isInvite)
        loginData.invite_token = decodeURIComponent(searchParams.get('invite_token') as string)
      const res = await login({
        url: '/login',
        body: loginData,
      })
      if (res.result === 'success') {
        if (res?.data?.access_token) {
          // Track login success event
          setWebAppAccessToken(res.data.access_token)
        }
        trackEvent('user_login_success', {
          method: 'email_password',
          is_invite: isInvite,
        })

        if (isInvite) {
          router.replace(`/signin/invite-settings?${searchParams.toString()}`)
        }
        else {
          const redirectUrl = resolvePostLoginRedirect()
          router.replace(redirectUrl || '/apps')
        }
      }
      else {
        toast.error(res.data)
      }
    }
    catch (error) {
      if ((error as ResponseError).code === 'authentication_failed') {
        toast.error(t('error.invalidEmailOrPassword', { ns: 'login' }))
      }
    }
    finally {
      setIsLoading(false)
    }
  }

  return (
    <form onSubmit={noop}>
      <div className="mb-3">
        <label htmlFor="email" className="my-2 text-text-secondary system-md-semibold">
          {t('email', { ns: 'login' })}
        </label>
        <div className="mt-1">
          <Input
            value={email}
            onChange={e => setEmail(e.target.value)}
            disabled={isInvite}
            id="email"
            type="email"
            autoComplete="email"
            placeholder={t('emailPlaceholder', { ns: 'login' }) || ''}
            tabIndex={1}
          />
        </div>
      </div>

      <div className="mb-3">
        <label htmlFor="password" className="my-2 flex items-center justify-between">
          <span className="text-text-secondary system-md-semibold">{t('password', { ns: 'login' })}</span>
          <Link
            href={`/reset-password?${searchParams.toString()}`}
            className={`system-xs-regular ${isEmailSetup ? 'text-components-button-secondary-accent-text' : 'pointer-events-none text-components-button-secondary-accent-text-disabled'}`}
            tabIndex={isEmailSetup ? 0 : -1}
            aria-disabled={!isEmailSetup}
          >
            {t('forget', { ns: 'login' })}
          </Link>
        </label>
        <div className="relative mt-1">
          <Input
            id="password"
            value={password}
            onChange={e => setPassword(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter')
                handleEmailPasswordLogin()
            }}
            type={showPassword ? 'text' : 'password'}
            autoComplete="current-password"
            placeholder={t('passwordPlaceholder', { ns: 'login' }) || ''}
            tabIndex={2}
          />
          <div className="absolute inset-y-0 right-0 flex items-center">
            <Button
              type="button"
              variant="ghost"
              onClick={() => setShowPassword(!showPassword)}
            >
              {showPassword ? '👀' : '😝'}
            </Button>
          </div>
        </div>
      </div>

      <div className="mb-2">
        <Button
          tabIndex={2}
          variant="primary"
          onClick={handleEmailPasswordLogin}
          disabled={isLoading || !email || !password}
          className="w-full"
        >
          {t('signBtn', { ns: 'login' })}
        </Button>
      </div>
    </form>
  )
}

```

### web/app/signin/components/social-auth.tsx
```tsx
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import { API_PREFIX } from '@/config'
import { useSearchParams } from '@/next/navigation'
import { getPurifyHref } from '@/utils'
import { cn } from '@/utils/classnames'
import style from '../page.module.css'

type SocialAuthProps = {
  disabled?: boolean
}

export default function SocialAuth(props: SocialAuthProps) {
  const { t } = useTranslation()
  const searchParams = useSearchParams()

  const getOAuthLink = (href: string) => {
    const url = getPurifyHref(`${API_PREFIX}${href}`)
    if (searchParams.has('invite_token'))
      return `${url}?${searchParams.toString()}`

    return url
  }
  return (
    <>
      <div className="w-full">
        <a href={getOAuthLink('/oauth/login/github')}>
          <Button
            disabled={props.disabled}
            className="w-full"
          >
            <>
              <span className={
                cn(style.githubIcon, 'mr-2 h-5 w-5')
              }
              />
              <span className="truncate leading-normal">{t('withGitHub', { ns: 'login' })}</span>
            </>
          </Button>
        </a>
      </div>
      <div className="w-full">
        <a href={getOAuthLink('/oauth/login/google')}>
          <Button
            disabled={props.disabled}
            className="w-full"
          >
            <>
              <span className={
                cn(style.googleIcon, 'mr-2 h-5 w-5')
              }
              />
              <span className="truncate leading-normal">{t('withGoogle', { ns: 'login' })}</span>
            </>
          </Button>
        </a>
      </div>
    </>
  )
}

```

### web/app/signin/components/mail-and-code-auth.tsx
```tsx
import type { FormEvent } from 'react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import { COUNT_DOWN_KEY, COUNT_DOWN_TIME_MS } from '@/app/components/signin/countdown'
import { emailRegex } from '@/config'
import { useLocale } from '@/context/i18n'
import { useRouter, useSearchParams } from '@/next/navigation'
import { sendEMailLoginCode } from '@/service/common'

type MailAndCodeAuthProps = {
  isInvite: boolean
}

export default function MailAndCodeAuth({ isInvite }: MailAndCodeAuthProps) {
  const { t } = useTranslation()
  const router = useRouter()
  const searchParams = useSearchParams()
  const emailFromLink = decodeURIComponent(searchParams.get('email') || '')
  const [email, setEmail] = useState(emailFromLink)
  const [loading, setIsLoading] = useState(false)
  const locale = useLocale()

  const handleGetEMailVerificationCode = async () => {
    try {
      if (!email) {
        toast.error(t('error.emailEmpty', { ns: 'login' }))
        return
      }

      if (!emailRegex.test(email)) {
        toast.error(t('error.emailInValid', { ns: 'login' }))
        return
      }
      setIsLoading(true)
      const ret = await sendEMailLoginCode(email, locale)
      if (ret.result === 'success') {
        localStorage.setItem(COUNT_DOWN_KEY, `${COUNT_DOWN_TIME_MS}`)
        const params = new URLSearchParams(searchParams)
        params.set('email', encodeURIComponent(email))
        params.set('token', encodeURIComponent(ret.data))
        router.push(`/signin/check-code?${params.toString()}`)
      }
    }
    catch (error) {
      console.error(error)
    }
    finally {
      setIsLoading(false)
    }
  }

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    handleGetEMailVerificationCode()
  }

  return (
    <form onSubmit={handleSubmit}>
      <input type="text" className="hidden" />
      <div className="mb-2">
        <label htmlFor="email" className="system-md-semibold my-2 text-text-secondary">{t('email', { ns: 'login' })}</label>
        <div className="mt-1">
          <Input id="email" type="email" disabled={isInvite} value={email} placeholder={t('emailPlaceholder', { ns: 'login' }) as string} onChange={e => setEmail(e.target.value)} />
        </div>
        <div className="mt-3">
          <Button type="submit" loading={loading} disabled={loading || !email} variant="primary" className="w-full">{t('signup.verifyMail', { ns: 'login' })}</Button>
        </div>
      </div>
    </form>
  )
}

```

### web/app/signin/components/sso-auth.tsx
```tsx
'use client'
import type { FC } from 'react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import { Lock01 } from '@/app/components/base/icons/src/vender/solid/security'
import { toast } from '@/app/components/base/ui/toast'
import { useRouter, useSearchParams } from '@/next/navigation'
import { getUserOAuth2SSOUrl, getUserOIDCSSOUrl, getUserSAMLSSOUrl } from '@/service/sso'
import { SSOProtocol } from '@/types/feature'

type SSOAuthProps = {
  protocol: SSOProtocol | ''
}

const SSOAuth: FC<SSOAuthProps> = ({
  protocol,
}) => {
  const router = useRouter()
  const { t } = useTranslation()
  const searchParams = useSearchParams()
  const invite_token = decodeURIComponent(searchParams.get('invite_token') || '')

  const [isLoading, setIsLoading] = useState(false)

  const handleSSOLogin = () => {
    setIsLoading(true)
    if (protocol === SSOProtocol.SAML) {
      getUserSAMLSSOUrl(invite_token).then((res) => {
        router.push(res.url)
      }).finally(() => {
        setIsLoading(false)
      })
    }
    else if (protocol === SSOProtocol.OIDC) {
      getUserOIDCSSOUrl(invite_token).then((res) => {
        document.cookie = `user-oidc-state=${res.state};Path=/`
        router.push(res.url)
      }).finally(() => {
        setIsLoading(false)
      })
    }
    else if (protocol === SSOProtocol.OAuth2) {
      getUserOAuth2SSOUrl(invite_token).then((res) => {
        document.cookie = `user-oauth2-state=${res.state};Path=/`
        router.push(res.url)
      }).finally(() => {
        setIsLoading(false)
      })
    }
    else {
      toast.error(t('error.invalidSSOProtocol', { ns: 'login' }))
      setIsLoading(false)
    }
  }

  return (
    <Button
      tabIndex={0}
      onClick={() => { handleSSOLogin() }}
      disabled={isLoading}
      className="w-full"
    >
      <Lock01 className="mr-2 h-5 w-5 text-text-accent-light-mode-only" />
      <span className="truncate">{t('withSSO', { ns: 'login' })}</span>
    </Button>
  )
}

export default SSOAuth

```

### web/app/signin/layout.tsx
```tsx
'use client'
import { useGlobalPublicStore } from '@/context/global-public-context'

import useDocumentTitle from '@/hooks/use-document-title'
import { cn } from '@/utils/classnames'
import Header from './_header'

export default function SignInLayout({ children }: any) {
  const { systemFeatures } = useGlobalPublicStore()
  useDocumentTitle('')
  return (
    <>
      <div className={cn('flex min-h-screen w-full justify-center bg-background-default-burn p-6')}>
        <div className={cn('flex w-full shrink-0 flex-col items-center rounded-2xl border border-effects-highlight bg-background-default-subtle')}>
          <Header />
          <div className={cn('flex w-full grow flex-col items-center justify-center px-6 md:px-[108px]')}>
            <div className="flex flex-col md:w-[400px]">
              {children}
            </div>
          </div>
          {systemFeatures.branding.enabled === false && (
            <div className="system-xs-regular px-8 py-6 text-text-tertiary">
              ©
              {' '}
              {new Date().getFullYear()}
              {' '}
              LangGenius, Inc. All rights reserved.
            </div>
          )}
        </div>
      </div>
    </>
  )
}

```

### web/app/signin/_header.tsx
```tsx
'use client'
import type { Locale } from '@/i18n-config'
import Divider from '@/app/components/base/divider'
import LocaleSigninSelect from '@/app/components/base/select/locale-signin'
import { useGlobalPublicStore } from '@/context/global-public-context'
import { useLocale } from '@/context/i18n'
import { setLocaleOnClient } from '@/i18n-config'
import { languages } from '@/i18n-config/language'
import dynamic from '@/next/dynamic'

// Avoid rendering the logo and theme selector on the server
const DifyLogo = dynamic(() => import('@/app/components/base/logo/dify-logo'), {
  ssr: false,
  loading: () => <div className="h-7 w-16 bg-transparent" />,
})
const ThemeSelector = dynamic(() => import('@/app/components/base/theme-selector'), {
  ssr: false,
  loading: () => <div className="size-8 bg-transparent" />,
})

const Header = () => {
  const locale = useLocale()
  const systemFeatures = useGlobalPublicStore(s => s.systemFeatures)

  return (
    <div className="flex w-full items-center justify-between p-6">
      {systemFeatures.branding.enabled && systemFeatures.branding.login_page_logo
        ? (
            <img
              src={systemFeatures.branding.login_page_logo}
              className="block h-7 w-auto object-contain"
              alt="logo"
            />
          )
        : <DifyLogo size="large" />}
      <div className="flex items-center gap-1">
        <LocaleSigninSelect
          value={locale}
          items={languages.filter(item => item.supported)}
          onChange={(value) => {
            setLocaleOnClient(value as Locale)
          }}
        />
        <Divider type="vertical" className="mx-0 ml-2 h-4" />
        <ThemeSelector />
      </div>
    </div>
  )
}

export default Header

```

### web/app/signin/one-more-step.tsx
```tsx
'use client'
import type { Reducer } from 'react'
import { useReducer } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import { SimpleSelect } from '@/app/components/base/select'
import Tooltip from '@/app/components/base/tooltip'
import { toast } from '@/app/components/base/ui/toast'
import { LICENSE_LINK } from '@/constants/link'
import { languages, LanguagesSupported } from '@/i18n-config/language'
import Link from '@/next/link'
import { useRouter, useSearchParams } from '@/next/navigation'
import { useOneMoreStep } from '@/service/use-common'
import { timezones } from '@/utils/timezone'
import Input from '../components/base/input'

type IState = {
  invitation_code: string
  interface_language: string
  timezone: string
}

type IAction
  = | { type: 'failed', payload: null }
    | { type: 'invitation_code', value: string }
    | { type: 'interface_language', value: string }
    | { type: 'timezone', value: string }

const reducer: Reducer<IState, IAction> = (state: IState, action: IAction) => {
  switch (action.type) {
    case 'invitation_code':
      return { ...state, invitation_code: action.value }
    case 'interface_language':
      return { ...state, interface_language: action.value }
    case 'timezone':
      return { ...state, timezone: action.value }
    case 'failed':
      return {
        invitation_code: '',
        interface_language: 'en-US',
        timezone: 'Asia/Shanghai',
      }
    default:
      throw new Error('Unknown action.')
  }
}

const OneMoreStep = () => {
  const { t } = useTranslation()
  const router = useRouter()
  const searchParams = useSearchParams()

  const [state, dispatch] = useReducer(reducer, {
    invitation_code: searchParams.get('invitation_code') || '',
    interface_language: 'en-US',
    timezone: 'Asia/Shanghai',
  })
  const { mutateAsync: submitOneMoreStep, isPending } = useOneMoreStep()

  const handleSubmit = async () => {
    if (isPending)
      return
    try {
      await submitOneMoreStep({
        invitation_code: state.invitation_code,
        interface_language: state.interface_language,
        timezone: state.timezone,
      })
      router.push('/apps')
    }
    catch (error: any) {
      if (error && error.status === 400)
        toast.error(t('invalidInvitationCode', { ns: 'login' }))
      dispatch({ type: 'failed', payload: null })
    }
  }

  return (
    <>
      <div className="mx-auto w-full">
        <h2 className="title-4xl-semi-bold text-text-secondary">{t('oneMoreStep', { ns: 'login' })}</h2>
        <p className="body-md-regular mt-1 text-text-tertiary">{t('createSample', { ns: 'login' })}</p>
      </div>

      <div className="mx-auto mt-6 w-full">
        <div className="relative">
          <div className="mb-5">
            <label className="system-md-semibold my-2 flex items-center justify-between text-text-secondary">
              {t('invitationCode', { ns: 'login' })}
              <Tooltip
                popupContent={(
                  <div className="w-[256px] text-xs font-medium">
                    <div className="font-medium">{t('sendUsMail', { ns: 'login' })}</div>
                    <div className="cursor-pointer text-xs font-medium text-text-accent-secondary">
                      <a href="mailto:request-invitation@langgenius.ai">request-invitation@langgenius.ai</a>
                    </div>
                  </div>
                )}
              >
                <span className="cursor-pointer text-text-accent-secondary">{t('dontHave', { ns: 'login' })}</span>
              </Tooltip>
            </label>
            <div className="mt-1">
              <Input
                id="invitation_code"
                value={state.invitation_code}
                type="text"
                placeholder={t('invitationCodePlaceholder', { ns: 'login' }) || ''}
                onChange={(e) => {
                  dispatch({ type: 'invitation_code', value: e.target.value.trim() })
                }}
              />
            </div>
          </div>
          <div className="mb-5">
            <label htmlFor="name" className="system-md-semibold my-2 text-text-secondary">
              {t('interfaceLanguage', { ns: 'login' })}
            </label>
            <div className="mt-1">
              <SimpleSelect
                defaultValue={LanguagesSupported[0]}
                items={languages.filter(item => item.supported)}
                onSelect={(item) => {
                  dispatch({ type: 'interface_language', value: item.value as typeof LanguagesSupported[number] })
                }}
              />
            </div>
          </div>
          <div className="mb-4">
            <label htmlFor="timezone" className="system-md-semibold text-text-tertiary">
              {t('timezone', { ns: 'login' })}
            </label>
            <div className="mt-1">
              <SimpleSelect
                defaultValue={state.timezone}
                items={timezones}
                onSelect={(item) => {
                  dispatch({ type: 'timezone', value: item.value as typeof state.timezone })
                }}
              />
            </div>
          </div>
          <div>
            <Button
              variant="primary"
              className="w-full"
              disabled={isPending}
              onClick={handleSubmit}
            >
              {t('go', { ns: 'login' })}
            </Button>
          </div>
          <div className="system-xs-regular mt-2 block w-full text-text-tertiary">
            {t('license.tip', { ns: 'login' })}
            &nbsp;
            <Link
              className="system-xs-medium text-text-accent-secondary"
              target="_blank"
              rel="noopener noreferrer"
              href={LICENSE_LINK}
            >
              {t('license.link', { ns: 'login' })}
            </Link>
          </div>
        </div>
      </div>
    </>
  )
}

export default OneMoreStep

```

### web/app/signin/split.tsx
```tsx
'use client'
import type { FC } from 'react'
import * as React from 'react'
import { cn } from '@/utils/classnames'

type Props = {
  className?: string
}

const Split: FC<Props> = ({
  className,
}) => {
  return (
    <div
      className={cn('h-px w-[400px] bg-[linear-gradient(90deg,rgba(255,255,255,0.01)_0%,rgba(16,24,40,0.08)_50.5%,rgba(255,255,255,0.01)_100%)]', className)}
    >
    </div>
  )
}
export default React.memo(Split)

```

### web/app/signin/page.tsx
```tsx
'use client'
import { useSearchParams } from '@/next/navigation'
import NormalForm from './normal-form'
import OneMoreStep from './one-more-step'

const SignIn = () => {
  const searchParams = useSearchParams()
  const step = searchParams.get('step')

  if (step === 'next')
    return <OneMoreStep />
  return <NormalForm />
}

export default SignIn

```

### api/controllers/console/auth/error.py
```py
from libs.exception import BaseHTTPException


class ApiKeyAuthFailedError(BaseHTTPException):
    error_code = "auth_failed"
    description = "{message}"
    code = 500


class InvalidEmailError(BaseHTTPException):
    error_code = "invalid_email"
    description = "The email address is not valid."
    code = 400


class PasswordMismatchError(BaseHTTPException):
    error_code = "password_mismatch"
    description = "The passwords do not match."
    code = 400


class InvalidTokenError(BaseHTTPException):
    error_code = "invalid_or_expired_token"
    description = "The token is invalid or has expired."
    code = 400


class PasswordResetRateLimitExceededError(BaseHTTPException):
    error_code = "password_reset_rate_limit_exceeded"
    description = "Too many password reset emails have been sent. Please try again in {minutes} minutes."
    code = 429

    def __init__(self, minutes: int = 1):
        description = self.description.format(minutes=int(minutes)) if self.description else None
        super().__init__(description=description)


class EmailRegisterRateLimitExceededError(BaseHTTPException):
    error_code = "email_register_rate_limit_exceeded"
    description = "Too many email register emails have been sent. Please try again in {minutes} minutes."
    code = 429

    def __init__(self, minutes: int = 1):
        description = self.description.format(minutes=int(minutes)) if self.description else None
        super().__init__(description=description)


class EmailChangeRateLimitExceededError(BaseHTTPException):
    error_code = "email_change_rate_limit_exceeded"
    description = "Too many email change emails have been sent. Please try again in {minutes} minutes."
    code = 429

    def __init__(self, minutes: int = 1):
        description = self.description.format(minutes=int(minutes)) if self.description else None
        super().__init__(description=description)


class OwnerTransferRateLimitExceededError(BaseHTTPException):
    error_code = "owner_transfer_rate_limit_exceeded"
    description = "Too many owner transfer emails have been sent. Please try again in {minutes} minutes."
    code = 429

    def __init__(self, minutes: int = 1):
        description = self.description.format(minutes=int(minutes)) if self.description else None
        super().__init__(description=description)


class EmailCodeError(BaseHTTPException):
    error_code = "email_code_error"
    description = "Email code is invalid or expired."
    code = 400


class EmailOrPasswordMismatchError(BaseHTTPException):
    error_code = "email_or_password_mismatch"
    description = "The email or password is mismatched."
    code = 400


class AuthenticationFailedError(BaseHTTPException):
    error_code = "authentication_failed"
    description = "Invalid email or password."
    code = 401


class EmailPasswordLoginLimitError(BaseHTTPException):
    error_code = "email_code_login_limit"
    description = "Too many incorrect password attempts. Please try again later."
    code = 429


class EmailCodeLoginRateLimitExceededError(BaseHTTPException):
    error_code = "email_code_login_rate_limit_exceeded"
    description = "Too many login emails have been sent. Please try again in {minutes} minutes."
    code = 429

    def __init__(self, minutes: int = 5):
        description = self.description.format(minutes=int(minutes)) if self.description else None
        super().__init__(description=description)


class EmailCodeAccountDeletionRateLimitExceededError(BaseHTTPException):
    error_code = "email_code_account_deletion_rate_limit_exceeded"
    description = "Too many account deletion emails have been sent. Please try again in {minutes} minutes."
    code = 429

    def __init__(self, minutes: int = 5):
        description = self.description.format(minutes=int(minutes)) if self.description else None
        super().__init__(description=description)


class EmailPasswordResetLimitError(BaseHTTPException):
    error_code = "email_password_reset_limit"
    description = "Too many failed password reset attempts. Please try again in 24 hours."
    code = 429


class EmailRegisterLimitError(BaseHTTPException):
    error_code = "email_register_limit"
    description = "Too many failed email register attempts. Please try again in 24 hours."
    code = 429


class EmailChangeLimitError(BaseHTTPException):
    error_code = "email_change_limit"
    description = "Too many failed email change attempts. Please try again in 24 hours."
    code = 429


class EmailAlreadyInUseError(BaseHTTPException):
    error_code = "email_already_in_use"
    description = "A user with this email already exists."
    code = 400


class OwnerTransferLimitError(BaseHTTPException):
    error_code = "owner_transfer_limit"
    description = "Too many failed owner transfer attempts. Please try again in 24 hours."
    code = 429


class NotOwnerError(BaseHTTPException):
    error_code = "not_owner"
    description = "You are not the owner of the workspace."
    code = 400


class CannotTransferOwnerToSelfError(BaseHTTPException):
    error_code = "cannot_transfer_owner_to_self"
    description = "You cannot transfer ownership to yourself."
    code = 400


class MemberNotInTenantError(BaseHTTPException):
    error_code = "member_not_in_tenant"
    description = "The member is not in the workspace."
    code = 400

```

### api/controllers/console/auth/data_source_bearer_auth.py
```py
from flask_restx import Resource
from pydantic import BaseModel, Field

from libs.login import current_account_with_tenant, login_required
from services.auth.api_key_auth_service import ApiKeyAuthService

from .. import console_ns
from ..auth.error import ApiKeyAuthFailedError
from ..wraps import account_initialization_required, is_admin_or_owner_required, setup_required

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class ApiKeyAuthBindingPayload(BaseModel):
    category: str = Field(...)
    provider: str = Field(...)
    credentials: dict = Field(...)


console_ns.schema_model(
    ApiKeyAuthBindingPayload.__name__,
    ApiKeyAuthBindingPayload.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0),
)


@console_ns.route("/api-key-auth/data-source")
class ApiKeyAuthDataSource(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    def get(self):
        _, current_tenant_id = current_account_with_tenant()
        data_source_api_key_bindings = ApiKeyAuthService.get_provider_auth_list(current_tenant_id)
        if data_source_api_key_bindings:
            return {
                "sources": [
                    {
                        "id": data_source_api_key_binding.id,
                        "category": data_source_api_key_binding.category,
                        "provider": data_source_api_key_binding.provider,
                        "disabled": data_source_api_key_binding.disabled,
                        "created_at": int(data_source_api_key_binding.created_at.timestamp()),
                        "updated_at": int(data_source_api_key_binding.updated_at.timestamp()),
                    }
                    for data_source_api_key_binding in data_source_api_key_bindings
                ]
            }
        return {"sources": []}


@console_ns.route("/api-key-auth/data-source/binding")
class ApiKeyAuthDataSourceBinding(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @is_admin_or_owner_required
    @console_ns.expect(console_ns.models[ApiKeyAuthBindingPayload.__name__])
    def post(self):
        # The role of the current user in the table must be admin or owner
        _, current_tenant_id = current_account_with_tenant()
        payload = ApiKeyAuthBindingPayload.model_validate(console_ns.payload)
        data = payload.model_dump()
        ApiKeyAuthService.validate_api_key_auth_args(data)
        try:
            ApiKeyAuthService.create_provider_auth(current_tenant_id, data)
        except Exception as e:
            raise ApiKeyAuthFailedError(str(e))
        return {"result": "success"}, 200


@console_ns.route("/api-key-auth/data-source/<uuid:binding_id>")
class ApiKeyAuthDataSourceBindingDelete(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @is_admin_or_owner_required
    def delete(self, binding_id):
        # The role of the current user in the table must be admin or owner
        _, current_tenant_id = current_account_with_tenant()

        ApiKeyAuthService.delete_provider_auth(current_tenant_id, binding_id)

        return {"result": "success"}, 204

```

### api/controllers/console/auth/email_register.py
```py
from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field, field_validator
from sqlalchemy.orm import sessionmaker

from configs import dify_config
from constants.languages import languages
from controllers.console import console_ns
from controllers.console.auth.error import (
    EmailAlreadyInUseError,
    EmailCodeError,
    EmailRegisterLimitError,
    InvalidEmailError,
    InvalidTokenError,
    PasswordMismatchError,
)
from extensions.ext_database import db
from libs.helper import EmailStr, extract_remote_ip
from libs.password import valid_password
from models import Account
from services.account_service import AccountService
from services.billing_service import BillingService
from services.errors.account import AccountNotFoundError, AccountRegisterError

from ..error import AccountInFreezeError, EmailSendIpLimitError
from ..wraps import email_password_login_enabled, email_register_enabled, setup_required

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class EmailRegisterSendPayload(BaseModel):
    email: EmailStr = Field(..., description="Email address")
    language: str | None = Field(default=None, description="Language code")


class EmailRegisterValidityPayload(BaseModel):
    email: EmailStr = Field(...)
    code: str = Field(...)
    token: str = Field(...)


class EmailRegisterResetPayload(BaseModel):
    token: str = Field(...)
    new_password: str = Field(...)
    password_confirm: str = Field(...)

    @field_validator("new_password", "password_confirm")
    @classmethod
    def validate_password(cls, value: str) -> str:
        return valid_password(value)


for model in (EmailRegisterSendPayload, EmailRegisterValidityPayload, EmailRegisterResetPayload):
    console_ns.schema_model(model.__name__, model.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


@console_ns.route("/email-register/send-email")
class EmailRegisterSendEmailApi(Resource):
    @setup_required
    @email_password_login_enabled
    @email_register_enabled
    def post(self):
        args = EmailRegisterSendPayload.model_validate(console_ns.payload)
        normalized_email = args.email.lower()

        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()
        language = "en-US"
        if args.language in languages:
            language = args.language

        if dify_config.BILLING_ENABLED and BillingService.is_email_in_freeze(normalized_email):
            raise AccountInFreezeError()

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(args.email, session=session)
        token = AccountService.send_email_register_email(email=normalized_email, account=account, language=language)
        return {"result": "success", "data": token}


@console_ns.route("/email-register/validity")
class EmailRegisterCheckApi(Resource):
    @setup_required
    @email_password_login_enabled
    @email_register_enabled
    def post(self):
        args = EmailRegisterValidityPayload.model_validate(console_ns.payload)

        user_email = args.email.lower()

        is_email_register_error_rate_limit = AccountService.is_email_register_error_rate_limit(user_email)
        if is_email_register_error_rate_limit:
            raise EmailRegisterLimitError()

        token_data = AccountService.get_email_register_data(args.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        normalized_token_email = token_email.lower() if isinstance(token_email, str) else token_email

        if user_email != normalized_token_email:
            raise InvalidEmailError()

        if args.code != token_data.get("code"):
            AccountService.add_email_register_error_rate_limit(user_email)
            raise EmailCodeError()

        # Verified, revoke the first token
        AccountService.revoke_email_register_token(args.token)

        # Refresh token data by generating a new token
        _, new_token = AccountService.generate_email_register_token(
            user_email, code=args.code, additional_data={"phase": "register"}
        )

        AccountService.reset_email_register_error_rate_limit(user_email)
        return {"is_valid": True, "email": normalized_token_email, "token": new_token}


@console_ns.route("/email-register")
class EmailRegisterResetApi(Resource):
    @setup_required
    @email_password_login_enabled
    @email_register_enabled
    def post(self):
        args = EmailRegisterResetPayload.model_validate(console_ns.payload)

        # Validate passwords match
        if args.new_password != args.password_confirm:
            raise PasswordMismatchError()

        # Validate token and get register data
        register_data = AccountService.get_email_register_data(args.token)
        if not register_data:
            raise InvalidTokenError()
        # Must use token in reset phase
        if register_data.get("phase", "") != "register":
            raise InvalidTokenError()

        # Revoke token to prevent reuse
        AccountService.revoke_email_register_token(args.token)

        email = register_data.get("email", "")
        normalized_email = email.lower()

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(email, session=session)

            if account:
                raise EmailAlreadyInUseError()
            else:
                account = self._create_new_account(normalized_email, args.password_confirm)
                if not account:
                    raise AccountNotFoundError()
                token_pair = AccountService.login(account=account, ip_address=extract_remote_ip(request))
                AccountService.reset_login_error_rate_limit(normalized_email)

        return {"result": "success", "data": token_pair.model_dump()}

    def _create_new_account(self, email: str, password: str) -> Account | None:
        # Create new account if allowed
        account = None
        try:
            account = AccountService.create_account_and_tenant(
                email=email,
                name=email,
                password=password,
                interface_language=languages[0],
            )
        except AccountRegisterError:
            raise AccountInFreezeError()

        return account

```

### api/controllers/console/auth/oauth_server.py
```py
from collections.abc import Callable
from functools import wraps
from typing import Concatenate

from flask import jsonify, request
from flask.typing import ResponseReturnValue
from flask_restx import Resource
from graphon.model_runtime.utils.encoders import jsonable_encoder
from pydantic import BaseModel
from werkzeug.exceptions import BadRequest, NotFound

from controllers.console.wraps import account_initialization_required, setup_required
from libs.login import current_account_with_tenant, login_required
from models import Account
from models.model import OAuthProviderApp
from services.oauth_server import OAUTH_ACCESS_TOKEN_EXPIRES_IN, OAuthGrantType, OAuthServerService

from .. import console_ns


class OAuthClientPayload(BaseModel):
    client_id: str


class OAuthProviderRequest(BaseModel):
    client_id: str
    redirect_uri: str


class OAuthTokenRequest(BaseModel):
    client_id: str
    grant_type: str
    code: str | None = None
    client_secret: str | None = None
    redirect_uri: str | None = None
    refresh_token: str | None = None


def oauth_server_client_id_required[T, **P, R](
    view: Callable[Concatenate[T, OAuthProviderApp, P], R],
) -> Callable[Concatenate[T, P], R]:
    @wraps(view)
    def decorated(self: T, *args: P.args, **kwargs: P.kwargs) -> R:
        json_data = request.get_json()
        if json_data is None:
            raise BadRequest("client_id is required")

        payload = OAuthClientPayload.model_validate(json_data)
        client_id = payload.client_id

        oauth_provider_app = OAuthServerService.get_oauth_provider_app(client_id)
        if not oauth_provider_app:
            raise NotFound("client_id is invalid")

        return view(self, oauth_provider_app, *args, **kwargs)

    return decorated


def oauth_server_access_token_required[T, **P, R](
    view: Callable[Concatenate[T, OAuthProviderApp, Account, P], R],
) -> Callable[Concatenate[T, OAuthProviderApp, P], R | ResponseReturnValue]:
    @wraps(view)
    def decorated(
        self: T, oauth_provider_app: OAuthProviderApp, *args: P.args, **kwargs: P.kwargs
    ) -> R | ResponseReturnValue:
        if not isinstance(oauth_provider_app, OAuthProviderApp):
            raise BadRequest("Invalid oauth_provider_app")

        authorization_header = request.headers.get("Authorization")
        if not authorization_header:
            response = jsonify({"error": "Authorization header is required"})
            response.status_code = 401
            response.headers["WWW-Authenticate"] = "Bearer"
            return response

        parts = authorization_header.strip().split(None, 1)
        if len(parts) != 2:
            response = jsonify({"error": "Invalid Authorization header format"})
            response.status_code = 401
            response.headers["WWW-Authenticate"] = "Bearer"
            return response

        token_type = parts[0].strip()
        if token_type.lower() != "bearer":
            response = jsonify({"error": "token_type is invalid"})
            response.status_code = 401
            response.headers["WWW-Authenticate"] = "Bearer"
            return response

        access_token = parts[1].strip()
        if not access_token:
            response = jsonify({"error": "access_token is required"})
            response.status_code = 401
            response.headers["WWW-Authenticate"] = "Bearer"
            return response

        account = OAuthServerService.validate_oauth_access_token(oauth_provider_app.client_id, access_token)
        if not account:
            response = jsonify({"error": "access_token or client_id is invalid"})
            response.status_code = 401
            response.headers["WWW-Authenticate"] = "Bearer"
            return response

        return view(self, oauth_provider_app, account, *args, **kwargs)

    return decorated


@console_ns.route("/oauth/provider")
class OAuthServerAppApi(Resource):
    @setup_required
    @oauth_server_client_id_required
    def post(self, oauth_provider_app: OAuthProviderApp):
        payload = OAuthProviderRequest.model_validate(request.get_json())
        redirect_uri = payload.redirect_uri

        # check if redirect_uri is valid
        if redirect_uri not in oauth_provider_app.redirect_uris:
            raise BadRequest("redirect_uri is invalid")

        return jsonable_encoder(
            {
                "app_icon": oauth_provider_app.app_icon,
                "app_label": oauth_provider_app.app_label,
                "scope": oauth_provider_app.scope,
            }
        )


@console_ns.route("/oauth/provider/authorize")
class OAuthServerUserAuthorizeApi(Resource):
    @setup_required
    @login_required
    @account_initialization_required
    @oauth_server_client_id_required
    def post(self, oauth_provider_app: OAuthProviderApp):
        current_user, _ = current_account_with_tenant()
        account = current_user
        user_account_id = account.id

        code = OAuthServerService.sign_oauth_authorization_code(oauth_provider_app.client_id, user_account_id)
        return jsonable_encoder(
            {
                "code": code,
            }
        )


@console_ns.route("/oauth/provider/token")
class OAuthServerUserTokenApi(Resource):
    @setup_required
    @oauth_server_client_id_required
    def post(self, oauth_provider_app: OAuthProviderApp):
        payload = OAuthTokenRequest.model_validate(request.get_json())

        try:
            grant_type = OAuthGrantType(payload.grant_type)
        except ValueError:
            raise BadRequest("invalid grant_type")
        match grant_type:
            case OAuthGrantType.AUTHORIZATION_CODE:
                if not payload.code:
                    raise BadRequest("code is required")

                if payload.client_secret != oauth_provider_app.client_secret:
                    raise BadRequest("client_secret is invalid")

                if payload.redirect_uri not in oauth_provider_app.redirect_uris:
                    raise BadRequest("redirect_uri is invalid")

                access_token, refresh_token = OAuthServerService.sign_oauth_access_token(
                    grant_type, code=payload.code, client_id=oauth_provider_app.client_id
                )
                return jsonable_encoder(
                    {
                        "access_token": access_token,
                        "token_type": "Bearer",
                        "expires_in": OAUTH_ACCESS_TOKEN_EXPIRES_IN,
                        "refresh_token": refresh_token,
                    }
                )
            case OAuthGrantType.REFRESH_TOKEN:
                if not payload.refresh_token:
                    raise BadRequest("refresh_token is required")

                access_token, refresh_token = OAuthServerService.sign_oauth_access_token(
                    grant_type, refresh_token=payload.refresh_token, client_id=oauth_provider_app.client_id
                )
                return jsonable_encoder(
                    {
                        "access_token": access_token,
                        "token_type": "Bearer",
                        "expires_in": OAUTH_ACCESS_TOKEN_EXPIRES_IN,
                        "refresh_token": refresh_token,
                    }
                )


@console_ns.route("/oauth/provider/account")
class OAuthServerUserAccountApi(Resource):
    @setup_required
    @oauth_server_client_id_required
    @oauth_server_access_token_required
    def post(self, oauth_provider_app: OAuthProviderApp, account: Account):
        return jsonable_encoder(
            {
                "name": account.name,
                "email": account.email,
                "avatar": account.avatar,
                "interface_language": account.interface_language,
                "timezone": account.timezone,
            }
        )

```

### api/controllers/console/auth/forgot_password.py
```py
import base64
import secrets

from flask import request
from flask_restx import Resource
from pydantic import BaseModel, Field, field_validator
from sqlalchemy.orm import sessionmaker

from controllers.common.schema import register_schema_models
from controllers.console import console_ns
from controllers.console.auth.error import (
    EmailCodeError,
    EmailPasswordResetLimitError,
    InvalidEmailError,
    InvalidTokenError,
    PasswordMismatchError,
)
from controllers.console.error import AccountNotFound, EmailSendIpLimitError
from controllers.console.wraps import email_password_login_enabled, setup_required
from events.tenant_event import tenant_was_created
from extensions.ext_database import db
from libs.helper import EmailStr, extract_remote_ip
from libs.password import hash_password, valid_password
from services.account_service import AccountService, TenantService
from services.feature_service import FeatureService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class ForgotPasswordSendPayload(BaseModel):
    email: EmailStr = Field(...)
    language: str | None = Field(default=None)


class ForgotPasswordCheckPayload(BaseModel):
    email: EmailStr = Field(...)
    code: str = Field(...)
    token: str = Field(...)


class ForgotPasswordResetPayload(BaseModel):
    token: str = Field(...)
    new_password: str = Field(...)
    password_confirm: str = Field(...)

    @field_validator("new_password", "password_confirm")
    @classmethod
    def validate_password(cls, value: str) -> str:
        return valid_password(value)


class ForgotPasswordEmailResponse(BaseModel):
    result: str = Field(description="Operation result")
    data: str | None = Field(default=None, description="Reset token")
    code: str | None = Field(default=None, description="Error code if account not found")


class ForgotPasswordCheckResponse(BaseModel):
    is_valid: bool = Field(description="Whether code is valid")
    email: EmailStr = Field(description="Email address")
    token: str = Field(description="New reset token")


class ForgotPasswordResetResponse(BaseModel):
    result: str = Field(description="Operation result")


register_schema_models(
    console_ns,
    ForgotPasswordSendPayload,
    ForgotPasswordCheckPayload,
    ForgotPasswordResetPayload,
    ForgotPasswordEmailResponse,
    ForgotPasswordCheckResponse,
    ForgotPasswordResetResponse,
)


@console_ns.route("/forgot-password")
class ForgotPasswordSendEmailApi(Resource):
    @console_ns.doc("send_forgot_password_email")
    @console_ns.doc(description="Send password reset email")
    @console_ns.expect(console_ns.models[ForgotPasswordSendPayload.__name__])
    @console_ns.response(
        200,
        "Email sent successfully",
        console_ns.models[ForgotPasswordEmailResponse.__name__],
    )
    @console_ns.response(400, "Invalid email or rate limit exceeded")
    @setup_required
    @email_password_login_enabled
    def post(self):
        args = ForgotPasswordSendPayload.model_validate(console_ns.payload)
        normalized_email = args.email.lower()

        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()

        if args.language is not None and args.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"

        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(args.email, session=session)

        token = AccountService.send_reset_password_email(
            account=account,
            email=normalized_email,
            language=language,
            is_allow_register=FeatureService.get_system_features().is_allow_register,
        )

        return {"result": "success", "data": token}


@console_ns.route("/forgot-password/validity")
class ForgotPasswordCheckApi(Resource):
    @console_ns.doc("check_forgot_password_code")
    @console_ns.doc(description="Verify password reset code")
    @console_ns.expect(console_ns.models[ForgotPasswordCheckPayload.__name__])
    @console_ns.response(
        200,
        "Code verified successfully",
        console_ns.models[ForgotPasswordCheckResponse.__name__],
    )
    @console_ns.response(400, "Invalid code or token")
    @setup_required
    @email_password_login_enabled
    def post(self):
        args = ForgotPasswordCheckPayload.model_validate(console_ns.payload)

        user_email = args.email.lower()

        is_forgot_password_error_rate_limit = AccountService.is_forgot_password_error_rate_limit(user_email)
        if is_forgot_password_error_rate_limit:
            raise EmailPasswordResetLimitError()

        token_data = AccountService.get_reset_password_data(args.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        if not isinstance(token_email, str):
            raise InvalidEmailError()
        normalized_token_email = token_email.lower()

        if user_email != normalized_token_email:
            raise InvalidEmailError()

        if args.code != token_data.get("code"):
            AccountService.add_forgot_password_error_rate_limit(user_email)
            raise EmailCodeError()

        # Verified, revoke the first token
        AccountService.revoke_reset_password_token(args.token)

        # Refresh token data by generating a new token
        _, new_token = AccountService.generate_reset_password_token(
            token_email, code=args.code, additional_data={"phase": "reset"}
        )

        AccountService.reset_forgot_password_error_rate_limit(user_email)
        return {"is_valid": True, "email": normalized_token_email, "token": new_token}


@console_ns.route("/forgot-password/resets")
class ForgotPasswordResetApi(Resource):
    @console_ns.doc("reset_password")
    @console_ns.doc(description="Reset password with verification token")
    @console_ns.expect(console_ns.models[ForgotPasswordResetPayload.__name__])
    @console_ns.response(
        200,
        "Password reset successfully",
        console_ns.models[ForgotPasswordResetResponse.__name__],
    )
    @console_ns.response(400, "Invalid token or password mismatch")
    @setup_required
    @email_password_login_enabled
    def post(self):
        args = ForgotPasswordResetPayload.model_validate(console_ns.payload)

        # Validate passwords match
        if args.new_password != args.password_confirm:
            raise PasswordMismatchError()

        # Validate token and get reset data
        reset_data = AccountService.get_reset_password_data(args.token)
        if not reset_data:
            raise InvalidTokenError()
        # Must use token in reset phase
        if reset_data.get("phase", "") != "reset":
            raise InvalidTokenError()

        # Revoke token to prevent reuse
        AccountService.revoke_reset_password_token(args.token)

        # Generate secure salt and hash password
        salt = secrets.token_bytes(16)
        password_hashed = hash_password(args.new_password, salt)

        email = reset_data.get("email", "")
        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(email, session=session)

            if account:
                self._update_existing_account(account, password_hashed, salt, session)
            else:
                raise AccountNotFound()

        return {"result": "success"}

    def _update_existing_account(self, account, password_hashed, salt, session):
        # Update existing account credentials
        account.password = base64.b64encode(password_hashed).decode()
        account.password_salt = base64.b64encode(salt).decode()

        # Create workspace if needed
        if (
            not TenantService.get_join_tenants(account)
            and FeatureService.get_system_features().is_allow_create_workspace
        ):
            tenant = TenantService.create_tenant(f"{account.name}'s Workspace")
            TenantService.create_tenant_member(tenant, account, role="owner")
            account.current_tenant = tenant
            tenant_was_created.send(tenant)

```

### api/controllers/console/auth/data_source_oauth.py
```py
import logging

import httpx
from flask import current_app, redirect, request
from flask_restx import Resource
from pydantic import BaseModel, Field

from configs import dify_config
from controllers.common.schema import register_schema_models
from libs.login import login_required
from libs.oauth_data_source import NotionOAuth

from .. import console_ns
from ..wraps import account_initialization_required, is_admin_or_owner_required, setup_required

logger = logging.getLogger(__name__)


class OAuthDataSourceResponse(BaseModel):
    data: str = Field(description="Authorization URL or 'internal' for internal setup")


class OAuthDataSourceBindingResponse(BaseModel):
    result: str = Field(description="Operation result")


class OAuthDataSourceSyncResponse(BaseModel):
    result: str = Field(description="Operation result")


register_schema_models(
    console_ns,
    OAuthDataSourceResponse,
    OAuthDataSourceBindingResponse,
    OAuthDataSourceSyncResponse,
)


def get_oauth_providers():
    with current_app.app_context():
        notion_oauth = NotionOAuth(
            client_id=dify_config.NOTION_CLIENT_ID or "",
            client_secret=dify_config.NOTION_CLIENT_SECRET or "",
            redirect_uri=dify_config.CONSOLE_API_URL + "/console/api/oauth/data-source/callback/notion",
        )

        OAUTH_PROVIDERS = {"notion": notion_oauth}
        return OAUTH_PROVIDERS


@console_ns.route("/oauth/data-source/<string:provider>")
class OAuthDataSource(Resource):
    @console_ns.doc("oauth_data_source")
    @console_ns.doc(description="Get OAuth authorization URL for data source provider")
    @console_ns.doc(params={"provider": "Data source provider name (notion)"})
    @console_ns.response(
        200,
        "Authorization URL or internal setup success",
        console_ns.models[OAuthDataSourceResponse.__name__],
    )
    @console_ns.response(400, "Invalid provider")
    @console_ns.response(403, "Admin privileges required")
    @is_admin_or_owner_required
    def get(self, provider: str):
        # The role of the current user in the table must be admin or owner
        OAUTH_DATASOURCE_PROVIDERS = get_oauth_providers()
        with current_app.app_context():
            oauth_provider = OAUTH_DATASOURCE_PROVIDERS.get(provider)
        if not oauth_provider:
            return {"error": "Invalid provider"}, 400
        if dify_config.NOTION_INTEGRATION_TYPE == "internal":
            internal_secret = dify_config.NOTION_INTERNAL_SECRET
            if not internal_secret:
                return ({"error": "Internal secret is not set"},)
            oauth_provider.save_internal_access_token(internal_secret)
            return {"data": "internal"}
        else:
            auth_url = oauth_provider.get_authorization_url()
            return {"data": auth_url}, 200


@console_ns.route("/oauth/data-source/callback/<string:provider>")
class OAuthDataSourceCallback(Resource):
    @console_ns.doc("oauth_data_source_callback")
    @console_ns.doc(description="Handle OAuth callback from data source provider")
    @console_ns.doc(
        params={
            "provider": "Data source provider name (notion)",
            "code": "Authorization code from OAuth provider",
            "error": "Error message from OAuth provider",
        }
    )
    @console_ns.response(302, "Redirect to console with result")
    @console_ns.response(400, "Invalid provider")
    def get(self, provider: str):
        OAUTH_DATASOURCE_PROVIDERS = get_oauth_providers()
        with current_app.app_context():
            oauth_provider = OAUTH_DATASOURCE_PROVIDERS.get(provider)
        if not oauth_provider:
            return {"error": "Invalid provider"}, 400
        if "code" in request.args:
            code = request.args.get("code")

            return redirect(f"{dify_config.CONSOLE_WEB_URL}?type=notion&code={code}")
        elif "error" in request.args:
            error = request.args.get("error")

            return redirect(f"{dify_config.CONSOLE_WEB_URL}?type=notion&error={error}")
        else:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}?type=notion&error=Access denied")


@console_ns.route("/oauth/data-source/binding/<string:provider>")
class OAuthDataSourceBinding(Resource):
    @console_ns.doc("oauth_data_source_binding")
    @console_ns.doc(description="Bind OAuth data source with authorization code")
    @console_ns.doc(
        params={"provider": "Data source provider name (notion)", "code": "Authorization code from OAuth provider"}
    )
    @console_ns.response(
        200,
        "Data source binding success",
        console_ns.models[OAuthDataSourceBindingResponse.__name__],
    )
    @console_ns.response(400, "Invalid provider or code")
    def get(self, provider: str):
        OAUTH_DATASOURCE_PROVIDERS = get_oauth_providers()
        with current_app.app_context():
            oauth_provider = OAUTH_DATASOURCE_PROVIDERS.get(provider)
        if not oauth_provider:
            return {"error": "Invalid provider"}, 400
        if "code" in request.args:
            code = request.args.get("code", "")
            if not code:
                return {"error": "Invalid code"}, 400
            try:
                oauth_provider.get_access_token(code)
            except httpx.HTTPStatusError as e:
                logger.exception(
                    "An error occurred during the OAuthCallback process with %s: %s", provider, e.response.text
                )
                return {"error": "OAuth data source process failed"}, 400

            return {"result": "success"}, 200


@console_ns.route("/oauth/data-source/<string:provider>/<uuid:binding_id>/sync")
class OAuthDataSourceSync(Resource):
    @console_ns.doc("oauth_data_source_sync")
    @console_ns.doc(description="Sync data from OAuth data source")
    @console_ns.doc(params={"provider": "Data source provider name (notion)", "binding_id": "Data source binding ID"})
    @console_ns.response(
        200,
        "Data source sync success",
        console_ns.models[OAuthDataSourceSyncResponse.__name__],
    )
    @console_ns.response(400, "Invalid provider or sync failed")
    @setup_required
    @login_required
    @account_initialization_required
    def get(self, provider, binding_id):
        provider = str(provider)
        binding_id = str(binding_id)
        OAUTH_DATASOURCE_PROVIDERS = get_oauth_providers()
        with current_app.app_context():
            oauth_provider = OAUTH_DATASOURCE_PROVIDERS.get(provider)
        if not oauth_provider:
            return {"error": "Invalid provider"}, 400
        try:
            oauth_provider.sync_data_source(binding_id)
        except httpx.HTTPStatusError as e:
            logger.exception(
                "An error occurred during the OAuthCallback process with %s: %s", provider, e.response.text
            )
            return {"error": "OAuth data source process failed"}, 400

        return {"result": "success"}, 200

```

### api/controllers/console/auth/login.py
```py
import flask_login
from flask import make_response, request
from flask_restx import Resource
from pydantic import BaseModel, Field

import services
from configs import dify_config
from constants.languages import get_valid_language
from controllers.console import console_ns
from controllers.console.auth.error import (
    AuthenticationFailedError,
    EmailCodeError,
    EmailPasswordLoginLimitError,
    InvalidEmailError,
    InvalidTokenError,
)
from controllers.console.error import (
    AccountBannedError,
    AccountInFreezeError,
    AccountNotFound,
    EmailSendIpLimitError,
    NotAllowedCreateWorkspace,
    WorkspacesLimitExceeded,
)
from controllers.console.wraps import (
    decrypt_code_field,
    decrypt_password_field,
    email_password_login_enabled,
    setup_required,
)
from events.tenant_event import tenant_was_created
from libs.helper import EmailStr, extract_remote_ip
from libs.login import current_account_with_tenant
from libs.token import (
    clear_access_token_from_cookie,
    clear_csrf_token_from_cookie,
    clear_refresh_token_from_cookie,
    extract_refresh_token,
    set_access_token_to_cookie,
    set_csrf_token_to_cookie,
    set_refresh_token_to_cookie,
)
from services.account_service import AccountService, InvitationDetailDict, RegisterService, TenantService
from services.billing_service import BillingService
from services.errors.account import AccountRegisterError
from services.errors.workspace import WorkSpaceNotAllowedCreateError, WorkspacesLimitExceededError
from services.feature_service import FeatureService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class LoginPayload(BaseModel):
    email: EmailStr = Field(..., description="Email address")
    password: str = Field(..., description="Password")
    remember_me: bool = Field(default=False, description="Remember me flag")
    invite_token: str | None = Field(default=None, description="Invitation token")


class EmailPayload(BaseModel):
    email: EmailStr = Field(...)
    language: str | None = Field(default=None)


class EmailCodeLoginPayload(BaseModel):
    email: EmailStr = Field(...)
    code: str = Field(...)
    token: str = Field(...)
    language: str | None = Field(default=None)


def reg(cls: type[BaseModel]):
    console_ns.schema_model(cls.__name__, cls.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


reg(LoginPayload)
reg(EmailPayload)
reg(EmailCodeLoginPayload)


@console_ns.route("/login")
class LoginApi(Resource):
    """Resource for user login."""

    @setup_required
    @email_password_login_enabled
    @console_ns.expect(console_ns.models[LoginPayload.__name__])
    @decrypt_password_field
    def post(self):
        """Authenticate user and login."""
        args = LoginPayload.model_validate(console_ns.payload)
        request_email = args.email
        normalized_email = request_email.lower()

        if dify_config.BILLING_ENABLED and BillingService.is_email_in_freeze(normalized_email):
            raise AccountInFreezeError()

        is_login_error_rate_limit = AccountService.is_login_error_rate_limit(normalized_email)
        if is_login_error_rate_limit:
            raise EmailPasswordLoginLimitError()

        invite_token = args.invite_token
        invitation_data: InvitationDetailDict | None = None
        if invite_token:
            invitation_data = RegisterService.get_invitation_with_case_fallback(None, request_email, invite_token)
            if invitation_data is None:
                invite_token = None

        try:
            if invitation_data:
                data = invitation_data.get("data", {})
                invitee_email = data.get("email") if data else None
                invitee_email_normalized = invitee_email.lower() if isinstance(invitee_email, str) else invitee_email
                if invitee_email_normalized != normalized_email:
                    raise InvalidEmailError()
            account = _authenticate_account_with_case_fallback(
                request_email, normalized_email, args.password, invite_token
            )
        except services.errors.account.AccountLoginError:
            raise AccountBannedError()
        except services.errors.account.AccountPasswordError as exc:
            AccountService.add_login_error_rate_limit(normalized_email)
            raise AuthenticationFailedError() from exc
        # SELF_HOSTED only have one workspace
        tenants = TenantService.get_join_tenants(account)
        if len(tenants) == 0:
            system_features = FeatureService.get_system_features()

            if system_features.is_allow_create_workspace and not system_features.license.workspaces.is_available():
                raise WorkspacesLimitExceeded()
            else:
                return {
                    "result": "fail",
                    "data": "workspace not found, please contact system admin to invite you to join in a workspace",
                }

        token_pair = AccountService.login(account=account, ip_address=extract_remote_ip(request))
        AccountService.reset_login_error_rate_limit(normalized_email)

        # Create response with cookies instead of returning tokens in body
        response = make_response({"result": "success"})

        set_access_token_to_cookie(request, response, token_pair.access_token)
        set_refresh_token_to_cookie(request, response, token_pair.refresh_token)
        set_csrf_token_to_cookie(request, response, token_pair.csrf_token)

        return response


@console_ns.route("/logout")
class LogoutApi(Resource):
    @setup_required
    def post(self):
        current_user, _ = current_account_with_tenant()
        account = current_user
        if isinstance(account, flask_login.AnonymousUserMixin):
            response = make_response({"result": "success"})
        else:
            AccountService.logout(account=account)
            flask_login.logout_user()
            response = make_response({"result": "success"})

        # Clear cookies on logout
        clear_access_token_from_cookie(response)
        clear_refresh_token_from_cookie(response)
        clear_csrf_token_from_cookie(response)

        return response


@console_ns.route("/reset-password")
class ResetPasswordSendEmailApi(Resource):
    @setup_required
    @email_password_login_enabled
    @console_ns.expect(console_ns.models[EmailPayload.__name__])
    def post(self):
        args = EmailPayload.model_validate(console_ns.payload)
        normalized_email = args.email.lower()

        if args.language is not None and args.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"
        try:
            account = _get_account_with_case_fallback(args.email)
        except AccountRegisterError:
            raise AccountInFreezeError()

        token = AccountService.send_reset_password_email(
            email=normalized_email,
            account=account,
            language=language,
            is_allow_register=FeatureService.get_system_features().is_allow_register,
        )

        return {"result": "success", "data": token}


@console_ns.route("/email-code-login")
class EmailCodeLoginSendEmailApi(Resource):
    @setup_required
    @console_ns.expect(console_ns.models[EmailPayload.__name__])
    def post(self):
        args = EmailPayload.model_validate(console_ns.payload)
        normalized_email = args.email.lower()

        ip_address = extract_remote_ip(request)
        if AccountService.is_email_send_ip_limit(ip_address):
            raise EmailSendIpLimitError()

        if args.language is not None and args.language == "zh-Hans":
            language = "zh-Hans"
        else:
            language = "en-US"
        try:
            account = _get_account_with_case_fallback(args.email)
        except AccountRegisterError:
            raise AccountInFreezeError()

        if account is None:
            if FeatureService.get_system_features().is_allow_register:
                token = AccountService.send_email_code_login_email(email=normalized_email, language=language)
            else:
                raise AccountNotFound()
        else:
            token = AccountService.send_email_code_login_email(account=account, language=language)

        return {"result": "success", "data": token}


@console_ns.route("/email-code-login/validity")
class EmailCodeLoginApi(Resource):
    @setup_required
    @console_ns.expect(console_ns.models[EmailCodeLoginPayload.__name__])
    @decrypt_code_field
    def post(self):
        args = EmailCodeLoginPayload.model_validate(console_ns.payload)

        original_email = args.email
        user_email = original_email.lower()
        language = args.language

        token_data = AccountService.get_email_code_login_data(args.token)
        if token_data is None:
            raise InvalidTokenError()

        token_email = token_data.get("email")
        normalized_token_email = token_email.lower() if isinstance(token_email, str) else token_email
        if normalized_token_email != user_email:
            raise InvalidEmailError()

        if token_data["code"] != args.code:
            raise EmailCodeError()

        AccountService.revoke_email_code_login_token(args.token)
        try:
            account = _get_account_with_case_fallback(original_email)
        except AccountRegisterError:
            raise AccountInFreezeError()
        if account:
            tenants = TenantService.get_join_tenants(account)
            if not tenants:
                workspaces = FeatureService.get_system_features().license.workspaces
                if not workspaces.is_available():
                    raise WorkspacesLimitExceeded()
                if not FeatureService.get_system_features().is_allow_create_workspace:
                    raise NotAllowedCreateWorkspace()
                else:
                    new_tenant = TenantService.create_tenant(f"{account.name}'s Workspace")
                    TenantService.create_tenant_member(new_tenant, account, role="owner")
                    account.current_tenant = new_tenant
                    tenant_was_created.send(new_tenant)

        if account is None:
            try:
                account = AccountService.create_account_and_tenant(
                    email=user_email,
                    name=user_email,
                    interface_language=get_valid_language(language),
                )
            except WorkSpaceNotAllowedCreateError:
                raise NotAllowedCreateWorkspace()
            except AccountRegisterError:
                raise AccountInFreezeError()
            except WorkspacesLimitExceededError:
                raise WorkspacesLimitExceeded()
        token_pair = AccountService.login(account, ip_address=extract_remote_ip(request))
        AccountService.reset_login_error_rate_limit(user_email)

        # Create response with cookies instead of returning tokens in body
        response = make_response({"result": "success"})

        set_csrf_token_to_cookie(request, response, token_pair.csrf_token)
        # Set HTTP-only secure cookies for tokens
        set_access_token_to_cookie(request, response, token_pair.access_token)
        set_refresh_token_to_cookie(request, response, token_pair.refresh_token)
        return response


@console_ns.route("/refresh-token")
class RefreshTokenApi(Resource):
    def post(self):
        # Get refresh token from cookie instead of request body
        refresh_token = extract_refresh_token(request)

        if not refresh_token:
            return {"result": "fail", "message": "No refresh token provided"}, 401

        try:
            new_token_pair = AccountService.refresh_token(refresh_token)

            # Create response with new cookies
            response = make_response({"result": "success"})

            # Update cookies with new tokens
            set_csrf_token_to_cookie(request, response, new_token_pair.csrf_token)
            set_access_token_to_cookie(request, response, new_token_pair.access_token)
            set_refresh_token_to_cookie(request, response, new_token_pair.refresh_token)
            return response
        except Exception as e:
            return {"result": "fail", "message": str(e)}, 401


def _get_account_with_case_fallback(email: str):
    account = AccountService.get_user_through_email(email)
    if account or email == email.lower():
        return account

    return AccountService.get_user_through_email(email.lower())


def _authenticate_account_with_case_fallback(
    original_email: str, normalized_email: str, password: str, invite_token: str | None
):
    try:
        return AccountService.authenticate(original_email, password, invite_token)
    except services.errors.account.AccountPasswordError:
        if original_email == normalized_email:
            raise
        return AccountService.authenticate(normalized_email, password, invite_token)

```

### api/controllers/console/auth/oauth.py
```py
import logging
import urllib.parse

import httpx
from flask import current_app, redirect, request
from flask_restx import Resource
from sqlalchemy.orm import sessionmaker
from werkzeug.exceptions import Unauthorized

from configs import dify_config
from constants.languages import languages
from events.tenant_event import tenant_was_created
from extensions.ext_database import db
from libs.datetime_utils import naive_utc_now
from libs.helper import extract_remote_ip
from libs.oauth import GitHubOAuth, GoogleOAuth, OAuthUserInfo
from libs.token import (
    set_access_token_to_cookie,
    set_csrf_token_to_cookie,
    set_refresh_token_to_cookie,
)
from models import Account, AccountStatus
from services.account_service import AccountService, RegisterService, TenantService
from services.billing_service import BillingService
from services.errors.account import AccountNotFoundError, AccountRegisterError
from services.errors.workspace import WorkSpaceNotAllowedCreateError, WorkSpaceNotFoundError
from services.feature_service import FeatureService

from .. import console_ns

logger = logging.getLogger(__name__)


def get_oauth_providers():
    with current_app.app_context():
        if not dify_config.GITHUB_CLIENT_ID or not dify_config.GITHUB_CLIENT_SECRET:
            github_oauth = None
        else:
            github_oauth = GitHubOAuth(
                client_id=dify_config.GITHUB_CLIENT_ID,
                client_secret=dify_config.GITHUB_CLIENT_SECRET,
                redirect_uri=dify_config.CONSOLE_API_URL + "/console/api/oauth/authorize/github",
            )
        if not dify_config.GOOGLE_CLIENT_ID or not dify_config.GOOGLE_CLIENT_SECRET:
            google_oauth = None
        else:
            google_oauth = GoogleOAuth(
                client_id=dify_config.GOOGLE_CLIENT_ID,
                client_secret=dify_config.GOOGLE_CLIENT_SECRET,
                redirect_uri=dify_config.CONSOLE_API_URL + "/console/api/oauth/authorize/google",
            )

        OAUTH_PROVIDERS = {"github": github_oauth, "google": google_oauth}
        return OAUTH_PROVIDERS


@console_ns.route("/oauth/login/<provider>")
class OAuthLogin(Resource):
    @console_ns.doc("oauth_login")
    @console_ns.doc(description="Initiate OAuth login process")
    @console_ns.doc(
        params={"provider": "OAuth provider name (github/google)", "invite_token": "Optional invitation token"}
    )
    @console_ns.response(302, "Redirect to OAuth authorization URL")
    @console_ns.response(400, "Invalid provider")
    def get(self, provider: str):
        invite_token = request.args.get("invite_token") or None
        OAUTH_PROVIDERS = get_oauth_providers()
        with current_app.app_context():
            oauth_provider = OAUTH_PROVIDERS.get(provider)
        if not oauth_provider:
            return {"error": "Invalid provider"}, 400

        auth_url = oauth_provider.get_authorization_url(invite_token=invite_token)
        return redirect(auth_url)


@console_ns.route("/oauth/authorize/<provider>")
class OAuthCallback(Resource):
    @console_ns.doc("oauth_callback")
    @console_ns.doc(description="Handle OAuth callback and complete login process")
    @console_ns.doc(
        params={
            "provider": "OAuth provider name (github/google)",
            "code": "Authorization code from OAuth provider",
            "state": "Optional state parameter (used for invite token)",
        }
    )
    @console_ns.response(302, "Redirect to console with access token")
    @console_ns.response(400, "OAuth process failed")
    def get(self, provider: str):
        OAUTH_PROVIDERS = get_oauth_providers()
        with current_app.app_context():
            oauth_provider = OAUTH_PROVIDERS.get(provider)
        if not oauth_provider:
            return {"error": "Invalid provider"}, 400

        code = request.args.get("code")
        state = request.args.get("state")
        invite_token = None
        if state:
            invite_token = state

        if not code:
            return {"error": "Authorization code is required"}, 400

        try:
            token = oauth_provider.get_access_token(code)
            user_info = oauth_provider.get_user_info(token)
        except httpx.RequestError as e:
            error_text = str(e)
            if isinstance(e, httpx.HTTPStatusError):
                error_text = e.response.text
            logger.exception("An error occurred during the OAuth process with %s: %s", provider, error_text)
            return {"error": "OAuth process failed"}, 400
        except ValueError as e:
            logger.warning("OAuth error with %s", provider, exc_info=True)
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message={urllib.parse.quote(str(e))}")

        if invite_token and RegisterService.is_valid_invite_token(invite_token):
            invitation = RegisterService.get_invitation_by_token(token=invite_token)
            if invitation:
                invitation_email = invitation.get("email", None)
                invitation_email_normalized = (
                    invitation_email.lower() if isinstance(invitation_email, str) else invitation_email
                )
                if invitation_email_normalized != user_info.email.lower():
                    return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message=Invalid invitation token.")

            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin/invite-settings?invite_token={invite_token}")

        try:
            account, oauth_new_user = _generate_account(provider, user_info)
        except AccountNotFoundError:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message=Account not found.")
        except (WorkSpaceNotFoundError, WorkSpaceNotAllowedCreateError):
            return redirect(
                f"{dify_config.CONSOLE_WEB_URL}/signin"
                "?message=Workspace not found, please contact system admin to invite you to join in a workspace."
            )
        except AccountRegisterError as e:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message={e.description}")

        # Check account status
        if account.status == AccountStatus.BANNED:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message=Account is banned.")

        if account.status == AccountStatus.PENDING:
            account.status = AccountStatus.ACTIVE
            account.initialized_at = naive_utc_now()
            db.session.commit()

        try:
            TenantService.create_owner_tenant_if_not_exist(account)
        except Unauthorized:
            return redirect(f"{dify_config.CONSOLE_WEB_URL}/signin?message=Workspace not found.")
        except WorkSpaceNotAllowedCreateError:
            return redirect(
                f"{dify_config.CONSOLE_WEB_URL}/signin"
                "?message=Workspace not found, please contact system admin to invite you to join in a workspace."
            )

        token_pair = AccountService.login(
            account=account,
            ip_address=extract_remote_ip(request),
        )

        base_url = dify_config.CONSOLE_WEB_URL
        query_char = "&" if "?" in base_url else "?"
        target_url = f"{base_url}{query_char}oauth_new_user={str(oauth_new_user).lower()}"
        response = redirect(target_url)

        set_access_token_to_cookie(request, response, token_pair.access_token)
        set_refresh_token_to_cookie(request, response, token_pair.refresh_token)
        set_csrf_token_to_cookie(request, response, token_pair.csrf_token)
        return response


def _get_account_by_openid_or_email(provider: str, user_info: OAuthUserInfo) -> Account | None:
    account: Account | None = Account.get_by_openid(provider, user_info.id)

    if not account:
        with sessionmaker(db.engine).begin() as session:
            account = AccountService.get_account_by_email_with_case_fallback(user_info.email, session=session)

    return account


def _generate_account(provider: str, user_info: OAuthUserInfo) -> tuple[Account, bool]:
    # Get account by openid or email.
    account = _get_account_by_openid_or_email(provider, user_info)
    oauth_new_user = False

    if account:
        tenants = TenantService.get_join_tenants(account)
        if not tenants:
            if not FeatureService.get_system_features().is_allow_create_workspace:
                raise WorkSpaceNotAllowedCreateError()
            else:
                new_tenant = TenantService.create_tenant(f"{account.name}'s Workspace")
                TenantService.create_tenant_member(new_tenant, account, role="owner")
                account.current_tenant = new_tenant
                tenant_was_created.send(new_tenant)

    if not account:
        normalized_email = user_info.email.lower()
        oauth_new_user = True
        if not FeatureService.get_system_features().is_allow_register:
            if dify_config.BILLING_ENABLED and BillingService.is_email_in_freeze(normalized_email):
                raise AccountRegisterError(
                    description=(
                        "This email account has been deleted within the past "
                        "30 days and is temporarily unavailable for new account registration"
                    )
                )
            else:
                raise AccountRegisterError(description=("Invalid email or password"))
        account_name = user_info.name or "Dify"
        account = RegisterService.register(
            email=normalized_email,
            name=account_name,
            password=None,
            open_id=user_info.id,
            provider=provider,
        )

        # Set interface language
        preferred_lang = request.accept_languages.best_match(languages)
        if preferred_lang and preferred_lang in languages:
            interface_language = preferred_lang
        else:
            interface_language = languages[0]
        account.interface_language = interface_language
        db.session.commit()

    # Link account
    AccountService.link_account_integrate(provider, user_info.id, account)

    return account, oauth_new_user

```

### api/controllers/console/auth/activate.py
```py
from flask import request
from flask_restx import Resource, fields
from pydantic import BaseModel, Field, field_validator

from constants.languages import supported_language
from controllers.console import console_ns
from controllers.console.error import AlreadyActivateError
from extensions.ext_database import db
from libs.datetime_utils import naive_utc_now
from libs.helper import EmailStr, timezone
from models import AccountStatus
from services.account_service import RegisterService

DEFAULT_REF_TEMPLATE_SWAGGER_2_0 = "#/definitions/{model}"


class ActivateCheckQuery(BaseModel):
    workspace_id: str | None = Field(default=None)
    email: EmailStr | None = Field(default=None)
    token: str


class ActivatePayload(BaseModel):
    workspace_id: str | None = Field(default=None)
    email: EmailStr | None = Field(default=None)
    token: str
    name: str = Field(..., max_length=30)
    interface_language: str = Field(...)
    timezone: str = Field(...)

    @field_validator("interface_language")
    @classmethod
    def validate_lang(cls, value: str) -> str:
        return supported_language(value)

    @field_validator("timezone")
    @classmethod
    def validate_tz(cls, value: str) -> str:
        return timezone(value)


for model in (ActivateCheckQuery, ActivatePayload):
    console_ns.schema_model(model.__name__, model.model_json_schema(ref_template=DEFAULT_REF_TEMPLATE_SWAGGER_2_0))


@console_ns.route("/activate/check")
class ActivateCheckApi(Resource):
    @console_ns.doc("check_activation_token")
    @console_ns.doc(description="Check if activation token is valid")
    @console_ns.expect(console_ns.models[ActivateCheckQuery.__name__])
    @console_ns.response(
        200,
        "Success",
        console_ns.model(
            "ActivationCheckResponse",
            {
                "is_valid": fields.Boolean(description="Whether token is valid"),
                "data": fields.Raw(description="Activation data if valid"),
            },
        ),
    )
    def get(self):
        args = ActivateCheckQuery.model_validate(request.args.to_dict(flat=True))  # type: ignore

        workspaceId = args.workspace_id
        token = args.token

        invitation = RegisterService.get_invitation_with_case_fallback(workspaceId, args.email, token)
        if invitation:
            data = invitation.get("data", {})
            tenant = invitation.get("tenant", None)

            # Check workspace permission
            if tenant:
                from libs.workspace_permission import check_workspace_member_invite_permission

                check_workspace_member_invite_permission(tenant.id)

            workspace_name = tenant.name if tenant else None
            workspace_id = tenant.id if tenant else None
            invitee_email = data.get("email") if data else None
            return {
                "is_valid": invitation is not None,
                "data": {"workspace_name": workspace_name, "workspace_id": workspace_id, "email": invitee_email},
            }
        else:
            return {"is_valid": False}


@console_ns.route("/activate")
class ActivateApi(Resource):
    @console_ns.doc("activate_account")
    @console_ns.doc(description="Activate account with invitation token")
    @console_ns.expect(console_ns.models[ActivatePayload.__name__])
    @console_ns.response(
        200,
        "Account activated successfully",
        console_ns.model(
            "ActivationResponse",
            {
                "result": fields.String(description="Operation result"),
            },
        ),
    )
    @console_ns.response(400, "Already activated or invalid token")
    def post(self):
        args = ActivatePayload.model_validate(console_ns.payload)

        normalized_request_email = args.email.lower() if args.email else None
        invitation = RegisterService.get_invitation_with_case_fallback(args.workspace_id, args.email, args.token)
        if invitation is None:
            raise AlreadyActivateError()

        RegisterService.revoke_token(args.workspace_id, normalized_request_email, args.token)

        account = invitation["account"]
        account.name = args.name

        account.interface_language = args.interface_language
        account.timezone = args.timezone
        account.interface_theme = "light"
        account.status = AccountStatus.ACTIVE
        account.initialized_at = naive_utc_now()
        db.session.commit()

        return {"result": "success"}

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-014.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
