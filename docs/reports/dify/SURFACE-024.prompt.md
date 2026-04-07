You are a security auditor. Analyze the following source code for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-024
- **Kind**: endpoint
- **Identifier**: Webapp Signin and Password Reset for End Users
- **Description**: End-user authentication for shared web apps with password reset. Risk of account enumeration, brute force on reset codes, and session fixation after password change.
- **Locations**: web/app/(shareLayout)/webapp-signin/, web/app/(shareLayout)/webapp-reset-password/, api/controllers/service_api/end_user/

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

### web/app/(shareLayout)/webapp-signin/normalForm.tsx
```tsx
'use client'
import { RiContractLine, RiDoorLockLine, RiErrorWarningFill } from '@remixicon/react'
import * as React from 'react'
import { useCallback, useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import Loading from '@/app/components/base/loading'
import { IS_CE_EDITION } from '@/config'
import { useGlobalPublicStore } from '@/context/global-public-context'
import Link from '@/next/link'
import { LicenseStatus } from '@/types/feature'
import { cn } from '@/utils/classnames'
import MailAndCodeAuth from './components/mail-and-code-auth'
import MailAndPasswordAuth from './components/mail-and-password-auth'
import SSOAuth from './components/sso-auth'

const NormalForm = () => {
  const { t } = useTranslation()

  const [isLoading, setIsLoading] = useState(true)
  const { systemFeatures } = useGlobalPublicStore()
  const [authType, updateAuthType] = useState<'code' | 'password'>('password')
  const [showORLine, setShowORLine] = useState(false)
  const [allMethodsAreDisabled, setAllMethodsAreDisabled] = useState(false)

  const init = useCallback(async () => {
    try {
      setAllMethodsAreDisabled(!systemFeatures.enable_social_oauth_login && !systemFeatures.enable_email_code_login && !systemFeatures.enable_email_password_login && !systemFeatures.sso_enforced_for_signin)
      setShowORLine((systemFeatures.enable_social_oauth_login || systemFeatures.sso_enforced_for_signin) && (systemFeatures.enable_email_code_login || systemFeatures.enable_email_password_login))
      updateAuthType(systemFeatures.enable_email_password_login ? 'password' : 'code')
    }
    catch (error) {
      console.error(error)
      setAllMethodsAreDisabled(true)
    }
    finally { setIsLoading(false) }
  }, [systemFeatures])
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
            <p className="system-sm-medium text-text-primary">{t('licenseLost', { ns: 'login' })}</p>
            <p className="system-xs-regular mt-1 text-text-tertiary">{t('licenseLostTip', { ns: 'login' })}</p>
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
            <p className="system-sm-medium text-text-primary">{t('licenseExpired', { ns: 'login' })}</p>
            <p className="system-xs-regular mt-1 text-text-tertiary">{t('licenseExpiredTip', { ns: 'login' })}</p>
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
            <p className="system-sm-medium text-text-primary">{t('licenseInactive', { ns: 'login' })}</p>
            <p className="system-xs-regular mt-1 text-text-tertiary">{t('licenseInactiveTip', { ns: 'login' })}</p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <>
      <div className="mx-auto mt-8 w-full">
        <div className="mx-auto w-full">
          <h2 className="title-4xl-semi-bold text-text-primary">{systemFeatures.branding.enabled ? t('pageTitleForE', { ns: 'login' }) : t('pageTitle', { ns: 'login' })}</h2>
          <p className="body-md-regular mt-2 text-text-tertiary">{t('welcome', { ns: 'login' })}</p>
        </div>
        <div className="relative">
          <div className="mt-6 flex flex-col gap-3">
            {systemFeatures.sso_enforced_for_signin && (
              <div className="w-full">
                <SSOAuth protocol={systemFeatures.sso_enforced_for_signin_protocol} />
              </div>
            )}
          </div>

          {showORLine && (
            <div className="relative mt-6">
              <div className="absolute inset-0 flex items-center" aria-hidden="true">
                <div className="h-px w-full bg-linear-to-r from-background-gradient-mask-transparent via-divider-regular to-background-gradient-mask-transparent"></div>
              </div>
              <div className="relative flex justify-center">
                <span className="system-xs-medium-uppercase px-2 text-text-tertiary">{t('or', { ns: 'login' })}</span>
              </div>
            </div>
          )}
          {
            (systemFeatures.enable_email_code_login || systemFeatures.enable_email_password_login) && (
              <>
                {systemFeatures.enable_email_code_login && authType === 'code' && (
                  <>
                    <MailAndCodeAuth />
                    {systemFeatures.enable_email_password_login && (
                      <div className="cursor-pointer py-1 text-center" onClick={() => { updateAuthType('password') }}>
                        <span className="system-xs-medium text-components-button-secondary-accent-text">{t('usePassword', { ns: 'login' })}</span>
                      </div>
                    )}
                  </>
                )}
                {systemFeatures.enable_email_password_login && authType === 'password' && (
                  <>
                    <MailAndPasswordAuth isEmailSetup={systemFeatures.is_email_setup} />
                    {systemFeatures.enable_email_code_login && (
                      <div className="cursor-pointer py-1 text-center" onClick={() => { updateAuthType('code') }}>
                        <span className="system-xs-medium text-components-button-secondary-accent-text">{t('useVerificationCode', { ns: 'login' })}</span>
                      </div>
                    )}
                  </>
                )}
              </>
            )
          }
          {allMethodsAreDisabled && (
            <>
              <div className="rounded-lg bg-linear-to-r from-workflow-workflow-progress-bg-1 to-workflow-workflow-progress-bg-2 p-4">
                <div className="shadows-shadow-lg mb-2 flex h-10 w-10 items-center justify-center rounded-xl bg-components-card-bg shadow">
                  <RiDoorLockLine className="h-5 w-5" />
                </div>
                <p className="system-sm-medium text-text-primary">{t('noLoginMethod', { ns: 'login' })}</p>
                <p className="system-xs-regular mt-1 text-text-tertiary">{t('noLoginMethodTip', { ns: 'login' })}</p>
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
              <div className="system-xs-regular mt-2 block w-full text-text-tertiary">
                {t('tosDesc', { ns: 'login' })}
              &nbsp;
                <Link
                  className="system-xs-medium text-text-secondary hover:underline"
                  target="_blank"
                  rel="noopener noreferrer"
                  href="https://dify.ai/terms"
                >
                  {t('tos', { ns: 'login' })}
                </Link>
              &nbsp;&&nbsp;
                <Link
                  className="system-xs-medium text-text-secondary hover:underline"
                  target="_blank"
                  rel="noopener noreferrer"
                  href="https://dify.ai/privacy"
                >
                  {t('pp', { ns: 'login' })}
                </Link>
              </div>
              {IS_CE_EDITION && (
                <div className="w-hull system-xs-regular mt-2 block text-text-tertiary">
                  {t('goToInit', { ns: 'login' })}
              &nbsp;
                  <Link
                    className="system-xs-medium text-text-secondary hover:underline"
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

### web/app/(shareLayout)/webapp-signin/check-code/page.tsx
```tsx
'use client'
import type { FormEvent } from 'react'
import { RiArrowLeftLine, RiMailSendFill } from '@remixicon/react'
import { useCallback, useEffect, useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import Countdown from '@/app/components/signin/countdown'
import { useLocale } from '@/context/i18n'
import { useWebAppStore } from '@/context/web-app-context'
import { useRouter, useSearchParams } from '@/next/navigation'
import { sendWebAppEMailLoginCode, webAppEmailLoginWithCode } from '@/service/common'
import { fetchAccessToken } from '@/service/share'
import { setWebAppAccessToken, setWebAppPassport } from '@/service/webapp-auth'
import { encryptVerificationCode } from '@/utils/encryption'

export default function CheckCode() {
  const { t } = useTranslation()
  const router = useRouter()
  const searchParams = useSearchParams()
  const email = decodeURIComponent(searchParams.get('email') as string)
  const token = decodeURIComponent(searchParams.get('token') as string)
  const [code, setVerifyCode] = useState('')
  const [loading, setIsLoading] = useState(false)
  const locale = useLocale()
  const codeInputRef = useRef<HTMLInputElement>(null)
  const redirectUrl = searchParams.get('redirect_url')
  const embeddedUserId = useWebAppStore(s => s.embeddedUserId)

  const getAppCodeFromRedirectUrl = useCallback(() => {
    if (!redirectUrl)
      return null
    const url = new URL(`${window.location.origin}${decodeURIComponent(redirectUrl)}`)
    const appCode = url.pathname.split('/').pop()
    if (!appCode)
      return null

    return appCode
  }, [redirectUrl])

  const verify = async () => {
    try {
      const appCode = getAppCodeFromRedirectUrl()
      if (!code.trim()) {
        toast.error(t('checkCode.emptyCode', { ns: 'login' }))
        return
      }
      if (!/\d{6}/.test(code)) {
        toast.error(t('checkCode.invalidCode', { ns: 'login' }))
        return
      }
      if (!redirectUrl || !appCode) {
        toast.error(t('error.redirectUrlMissing', { ns: 'login' }))
        return
      }
      setIsLoading(true)
      const ret = await webAppEmailLoginWithCode({ email, code: encryptVerificationCode(code), token })
      if (ret.result === 'success') {
        if (ret?.data?.access_token) {
          setWebAppAccessToken(ret.data.access_token)
        }
        const { access_token } = await fetchAccessToken({
          appCode: appCode!,
          userId: embeddedUserId || undefined,
        })
        setWebAppPassport(appCode!, access_token)
        router.replace(decodeURIComponent(redirectUrl))
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
      const ret = await sendWebAppEMailLoginCode(email, locale)
      if (ret.result === 'success') {
        const params = new URLSearchParams(searchParams)
        params.set('token', encodeURIComponent(ret.data))
        router.replace(`/webapp-signin/check-code?${params.toString()}`)
      }
    }
    catch (error) { console.error(error) }
  }

  return (
    <div className="flex w-[400px] flex-col gap-3">
      <div className="inline-flex h-14 w-14 items-center justify-center rounded-2xl border border-components-panel-border-subtle bg-background-default-dodge shadow-lg">
        <RiMailSendFill className="h-6 w-6 text-2xl text-text-accent-light-mode-only" />
      </div>
      <div className="pb-4 pt-2">
        <h2 className="title-4xl-semi-bold text-text-primary">{t('checkCode.checkYourEmail', { ns: 'login' })}</h2>
        <p className="body-md-regular mt-2 text-text-secondary">
          <span>
            {t('checkCode.tipsPrefix', { ns: 'login' })}
            <strong>{email}</strong>
          </span>
          <br />
          {t('checkCode.validTime', { ns: 'login' })}
        </p>
      </div>

      <form onSubmit={handleSubmit}>
        <label htmlFor="code" className="system-md-semibold mb-1 text-text-secondary">{t('checkCode.verificationCode', { ns: 'login' })}</label>
        <Input
          ref={codeInputRef}
          id="code"
          value={code}
          onChange={e => setVerifyCode(e.target.value)}
          maxLength={6}
          className="mt-1"
          placeholder={t('checkCode.verificationCodePlaceholder', { ns: 'login' }) || ''}
        />
        <Button type="submit" loading={loading} disabled={loading} className="my-3 w-full" variant="primary">{t('checkCode.verify', { ns: 'login' })}</Button>
        <Countdown onResend={resendCode} />
      </form>
      <div className="py-2">
        <div className="h-px bg-linear-to-r from-background-gradient-mask-transparent via-divider-regular to-background-gradient-mask-transparent"></div>
      </div>
      <div onClick={() => router.back()} className="flex h-9 cursor-pointer items-center justify-center text-text-tertiary">
        <div className="bg-background-default-dimm inline-block rounded-full p-1">
          <RiArrowLeftLine size={12} />
        </div>
        <span className="system-xs-regular ml-2">{t('back', { ns: 'login' })}</span>
      </div>
    </div>
  )
}

```

### web/app/(shareLayout)/webapp-signin/components/external-member-sso-auth.tsx
```tsx
'use client'
import * as React from 'react'
import { useCallback, useEffect } from 'react'
import AppUnavailable from '@/app/components/base/app-unavailable'
import Loading from '@/app/components/base/loading'
import { toast } from '@/app/components/base/ui/toast'
import { useGlobalPublicStore } from '@/context/global-public-context'
import { useRouter, useSearchParams } from '@/next/navigation'
import { fetchWebOAuth2SSOUrl, fetchWebOIDCSSOUrl, fetchWebSAMLSSOUrl } from '@/service/share'
import { SSOProtocol } from '@/types/feature'

const ExternalMemberSSOAuth = () => {
  const systemFeatures = useGlobalPublicStore(s => s.systemFeatures)
  const searchParams = useSearchParams()
  const router = useRouter()

  const redirectUrl = searchParams.get('redirect_url')

  const showErrorToast = (message: string) => {
    toast.error(message)
  }

  const getAppCodeFromRedirectUrl = useCallback(() => {
    if (!redirectUrl)
      return null
    const url = new URL(`${window.location.origin}${decodeURIComponent(redirectUrl)}`)
    const appCode = url.pathname.split('/').pop()
    if (!appCode)
      return null

    return appCode
  }, [redirectUrl])

  const handleSSOLogin = useCallback(async () => {
    const appCode = getAppCodeFromRedirectUrl()
    if (!appCode || !redirectUrl) {
      showErrorToast('redirect url or app code is invalid.')
      return
    }

    switch (systemFeatures.webapp_auth.sso_config.protocol) {
      case SSOProtocol.SAML: {
        const samlRes = await fetchWebSAMLSSOUrl(appCode, redirectUrl)
        router.push(samlRes.url)
        break
      }
      case SSOProtocol.OIDC: {
        const oidcRes = await fetchWebOIDCSSOUrl(appCode, redirectUrl)
        router.push(oidcRes.url)
        break
      }
      case SSOProtocol.OAuth2: {
        const oauth2Res = await fetchWebOAuth2SSOUrl(appCode, redirectUrl)
        router.push(oauth2Res.url)
        break
      }
      case '':
        break
      default:
        showErrorToast('SSO protocol is not supported.')
    }
  }, [getAppCodeFromRedirectUrl, redirectUrl, router, systemFeatures.webapp_auth.sso_config.protocol])

  useEffect(() => {
    handleSSOLogin()
  }, [handleSSOLogin])

  if (!systemFeatures.webapp_auth.sso_config.protocol) {
    return (
      <div className="flex h-full items-center justify-center">
        <AppUnavailable code={403} unknownReason="sso protocol is invalid." />
      </div>
    )
  }

  return (
    <div className="flex h-full items-center justify-center">
      <Loading />
    </div>
  )
}

export default React.memo(ExternalMemberSSOAuth)

```

### web/app/(shareLayout)/webapp-signin/components/mail-and-password-auth.tsx
```tsx
'use client'
import { noop } from 'es-toolkit/function'
import { useCallback, useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import { emailRegex } from '@/config'
import { useLocale } from '@/context/i18n'
import { useWebAppStore } from '@/context/web-app-context'
import Link from '@/next/link'
import { useRouter, useSearchParams } from '@/next/navigation'
import { webAppLogin } from '@/service/common'
import { fetchAccessToken } from '@/service/share'
import { setWebAppAccessToken, setWebAppPassport } from '@/service/webapp-auth'
import { encryptPassword } from '@/utils/encryption'

type MailAndPasswordAuthProps = {
  isEmailSetup: boolean
}

export default function MailAndPasswordAuth({ isEmailSetup }: MailAndPasswordAuthProps) {
  const { t } = useTranslation()
  const locale = useLocale()
  const router = useRouter()
  const searchParams = useSearchParams()
  const [showPassword, setShowPassword] = useState(false)
  const emailFromLink = decodeURIComponent(searchParams.get('email') || '')
  const [email, setEmail] = useState(emailFromLink)
  const [password, setPassword] = useState('')

  const [isLoading, setIsLoading] = useState(false)
  const redirectUrl = searchParams.get('redirect_url')
  const embeddedUserId = useWebAppStore(s => s.embeddedUserId)

  const getAppCodeFromRedirectUrl = useCallback(() => {
    if (!redirectUrl)
      return null
    const url = new URL(`${window.location.origin}${decodeURIComponent(redirectUrl)}`)
    const appCode = url.pathname.split('/').pop()
    if (!appCode)
      return null

    return appCode
  }, [redirectUrl])
  const appCode = getAppCodeFromRedirectUrl()
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

    if (!redirectUrl || !appCode) {
      toast.error(t('error.redirectUrlMissing', { ns: 'login' }))
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

      const res = await webAppLogin({
        url: '/login',
        body: loginData,
      })
      if (res.result === 'success') {
        if (res?.data?.access_token) {
          setWebAppAccessToken(res.data.access_token)
        }

        const { access_token } = await fetchAccessToken({
          appCode: appCode!,
          userId: embeddedUserId || undefined,
        })
        setWebAppPassport(appCode!, access_token)
        router.replace(decodeURIComponent(redirectUrl))
      }
      else {
        toast.error(res.data)
      }
    }
    catch (e: any) {
      if (e.code === 'authentication_failed')
        toast.error(e.message)
    }
    finally {
      setIsLoading(false)
    }
  }

  return (
    <form onSubmit={noop}>
      <div className="mb-3">
        <label htmlFor="email" className="system-md-semibold my-2 text-text-secondary">
          {t('email', { ns: 'login' })}
        </label>
        <div className="mt-1">
          <Input
            value={email}
            onChange={e => setEmail(e.target.value)}
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
          <span className="system-md-semibold text-text-secondary">{t('password', { ns: 'login' })}</span>
          <Link
            href={`/webapp-reset-password?${searchParams.toString()}`}
            className={`system-xs-regular ${isEmailSetup ? 'text-components-button-secondary-accent-text' : 'pointer-events-none text-components-button-secondary-accent-text-disabled'}`}
            tabIndex={isEmailSetup ? 0 : -1}
            aria-disabled={!isEmailSetup}
          >
            {t('forget', { ns: 'login' })}
          </Link>
        </label>
        <div className="relative mt-1">
          <Input
            value={password}
            onChange={e => setPassword(e.target.value)}
            id="password"
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

### web/app/(shareLayout)/webapp-signin/components/mail-and-code-auth.tsx
```tsx
import { noop } from 'es-toolkit/function'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import { COUNT_DOWN_KEY, COUNT_DOWN_TIME_MS } from '@/app/components/signin/countdown'
import { emailRegex } from '@/config'
import { useLocale } from '@/context/i18n'
import { useRouter, useSearchParams } from '@/next/navigation'
import { sendWebAppEMailLoginCode } from '@/service/common'

export default function MailAndCodeAuth() {
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
      const ret = await sendWebAppEMailLoginCode(email, locale)
      if (ret.result === 'success') {
        localStorage.setItem(COUNT_DOWN_KEY, `${COUNT_DOWN_TIME_MS}`)
        const params = new URLSearchParams(searchParams)
        params.set('email', encodeURIComponent(email))
        params.set('token', encodeURIComponent(ret.data))
        router.push(`/webapp-signin/check-code?${params.toString()}`)
      }
    }
    catch (error) {
      console.error(error)
    }
    finally {
      setIsLoading(false)
    }
  }

  return (
    <form onSubmit={noop}>
      <input type="text" className="hidden" />
      <div className="mb-2">
        <label htmlFor="email" className="system-md-semibold my-2 text-text-secondary">{t('email', { ns: 'login' })}</label>
        <div className="mt-1">
          <Input id="email" type="email" value={email} placeholder={t('emailPlaceholder', { ns: 'login' }) as string} onChange={e => setEmail(e.target.value)} />
        </div>
        <div className="mt-3">
          <Button loading={loading} disabled={loading || !email} variant="primary" className="w-full" onClick={handleGetEMailVerificationCode}>{t('signup.verifyMail', { ns: 'login' })}</Button>
        </div>
      </div>
    </form>
  )
}

```

### web/app/(shareLayout)/webapp-signin/components/sso-auth.tsx
```tsx
'use client'
import type { FC } from 'react'
import { useCallback, useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import { Lock01 } from '@/app/components/base/icons/src/vender/solid/security'
import { toast } from '@/app/components/base/ui/toast'
import { useRouter, useSearchParams } from '@/next/navigation'
import { fetchMembersOAuth2SSOUrl, fetchMembersOIDCSSOUrl, fetchMembersSAMLSSOUrl } from '@/service/share'
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

  const redirectUrl = searchParams.get('redirect_url')
  const getAppCodeFromRedirectUrl = useCallback(() => {
    if (!redirectUrl)
      return null
    const url = new URL(`${window.location.origin}${decodeURIComponent(redirectUrl)}`)
    const appCode = url.pathname.split('/').pop()
    if (!appCode)
      return null

    return appCode
  }, [redirectUrl])

  const [isLoading, setIsLoading] = useState(false)

  const handleSSOLogin = () => {
    const appCode = getAppCodeFromRedirectUrl()
    if (!redirectUrl || !appCode) {
      toast.error(t('error.invalidRedirectUrlOrAppCode', { ns: 'login' }))
      return
    }
    setIsLoading(true)
    if (protocol === SSOProtocol.SAML) {
      fetchMembersSAMLSSOUrl(appCode, redirectUrl).then((res) => {
        router.push(res.url)
      }).finally(() => {
        setIsLoading(false)
      })
    }
    else if (protocol === SSOProtocol.OIDC) {
      fetchMembersOIDCSSOUrl(appCode, redirectUrl).then((res) => {
        router.push(res.url)
      }).finally(() => {
        setIsLoading(false)
      })
    }
    else if (protocol === SSOProtocol.OAuth2) {
      fetchMembersOAuth2SSOUrl(appCode, redirectUrl).then((res) => {
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

### web/app/(shareLayout)/webapp-signin/layout.tsx
```tsx
'use client'

import type { PropsWithChildren } from 'react'
import { useTranslation } from 'react-i18next'
import { useGlobalPublicStore } from '@/context/global-public-context'
import useDocumentTitle from '@/hooks/use-document-title'
import { cn } from '@/utils/classnames'

export default function SignInLayout({ children }: PropsWithChildren) {
  const { t } = useTranslation()
  const systemFeatures = useGlobalPublicStore(s => s.systemFeatures)
  useDocumentTitle(t('webapp.login', { ns: 'login' }))
  return (
    <>
      <div className={cn('flex min-h-screen w-full justify-center bg-background-default-burn p-6')}>
        <div className={cn('flex w-full shrink-0 flex-col rounded-2xl border border-effects-highlight bg-background-default-subtle')}>
          {/* <Header /> */}
          <div className={cn('flex w-full grow flex-col items-center justify-center px-6 md:px-[108px]')}>
            <div className="flex justify-center md:w-[440px] lg:w-[600px]">
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

### web/app/(shareLayout)/webapp-signin/page.tsx
```tsx
'use client'
import type { FC } from 'react'
import * as React from 'react'
import { useCallback } from 'react'
import { useTranslation } from 'react-i18next'
import AppUnavailable from '@/app/components/base/app-unavailable'
import { useGlobalPublicStore } from '@/context/global-public-context'
import { useWebAppStore } from '@/context/web-app-context'
import { AccessMode } from '@/models/access-control'
import { useRouter, useSearchParams } from '@/next/navigation'
import { webAppLogout } from '@/service/webapp-auth'
import ExternalMemberSsoAuth from './components/external-member-sso-auth'
import NormalForm from './normalForm'

const WebSSOForm: FC = () => {
  const { t } = useTranslation()
  const systemFeatures = useGlobalPublicStore(s => s.systemFeatures)
  const webAppAccessMode = useWebAppStore(s => s.webAppAccessMode)
  const searchParams = useSearchParams()
  const router = useRouter()

  const redirectUrl = searchParams.get('redirect_url')

  const getSigninUrl = useCallback(() => {
    const params = new URLSearchParams()
    params.append('redirect_url', redirectUrl || '')
    return `/webapp-signin?${params.toString()}`
  }, [redirectUrl])

  const shareCode = useWebAppStore(s => s.shareCode)
  const backToHome = useCallback(async () => {
    await webAppLogout(shareCode!)
    const url = getSigninUrl()
    router.replace(url)
  }, [getSigninUrl, router, webAppLogout, shareCode])

  if (!redirectUrl) {
    return (
      <div className="flex h-full items-center justify-center">
        <AppUnavailable code={t('common.appUnavailable', { ns: 'share' })} unknownReason="redirect url is invalid." />
      </div>
    )
  }

  if (!systemFeatures.webapp_auth.enabled) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="system-xs-regular text-text-tertiary">{t('webapp.disabled', { ns: 'login' })}</p>
      </div>
    )
  }
  if (webAppAccessMode && (webAppAccessMode === AccessMode.ORGANIZATION || webAppAccessMode === AccessMode.SPECIFIC_GROUPS_MEMBERS)) {
    return (
      <div className="w-full max-w-[400px]">
        <NormalForm />
      </div>
    )
  }

  if (webAppAccessMode && webAppAccessMode === AccessMode.EXTERNAL_MEMBERS)
    return <ExternalMemberSsoAuth />

  return (
    <div className="flex h-full flex-col items-center justify-center gap-y-4">
      <AppUnavailable className="h-auto w-auto" isUnknownReason={true} />
      <span className="system-sm-regular cursor-pointer text-text-tertiary" onClick={backToHome}>{t('login.backToHome', { ns: 'share' })}</span>
    </div>
  )
}

export default React.memo(WebSSOForm)

```

### web/app/(shareLayout)/webapp-reset-password/check-code/page.tsx
```tsx
'use client'
import { RiArrowLeftLine, RiMailSendFill } from '@remixicon/react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import Countdown from '@/app/components/signin/countdown'
import { useLocale } from '@/context/i18n'

import { useRouter, useSearchParams } from '@/next/navigation'
import { sendWebAppResetPasswordCode, verifyWebAppResetPasswordCode } from '@/service/common'

export default function CheckCode() {
  const { t } = useTranslation()
  const router = useRouter()
  const searchParams = useSearchParams()
  const email = decodeURIComponent(searchParams.get('email') as string)
  const token = decodeURIComponent(searchParams.get('token') as string)
  const [code, setVerifyCode] = useState('')
  const [loading, setIsLoading] = useState(false)
  const locale = useLocale()

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
      const ret = await verifyWebAppResetPasswordCode({ email, code, token })
      if (ret.is_valid) {
        const params = new URLSearchParams(searchParams)
        params.set('token', encodeURIComponent(ret.token))
        router.push(`/webapp-reset-password/set-password?${params.toString()}`)
      }
    }
    catch (error) { console.error(error) }
    finally {
      setIsLoading(false)
    }
  }

  const resendCode = async () => {
    try {
      const res = await sendWebAppResetPasswordCode(email, locale)
      if (res.result === 'success') {
        const params = new URLSearchParams(searchParams)
        params.set('token', encodeURIComponent(res.data))
        router.replace(`/webapp-reset-password/check-code?${params.toString()}`)
      }
    }
    catch (error) { console.error(error) }
  }

  return (
    <div className="flex flex-col gap-3">
      <div className="inline-flex h-14 w-14 items-center justify-center rounded-2xl border border-components-panel-border-subtle bg-background-default-dodge text-text-accent-light-mode-only shadow-lg">
        <RiMailSendFill className="h-6 w-6 text-2xl" />
      </div>
      <div className="pb-4 pt-2">
        <h2 className="title-4xl-semi-bold text-text-primary">{t('checkCode.checkYourEmail', { ns: 'login' })}</h2>
        <p className="body-md-regular mt-2 text-text-secondary">
          <span>
            {t('checkCode.tipsPrefix', { ns: 'login' })}
            <strong>{email}</strong>
          </span>
          <br />
          {t('checkCode.validTime', { ns: 'login' })}
        </p>
      </div>

      <form action="">
        <input type="text" className="hidden" />
        <label htmlFor="code" className="system-md-semibold mb-1 text-text-secondary">{t('checkCode.verificationCode', { ns: 'login' })}</label>
        <Input value={code} onChange={e => setVerifyCode(e.target.value)} maxLength={6} className="mt-1" placeholder={t('checkCode.verificationCodePlaceholder', { ns: 'login' }) || ''} />
        <Button loading={loading} disabled={loading} className="my-3 w-full" variant="primary" onClick={verify}>{t('checkCode.verify', { ns: 'login' })}</Button>
        <Countdown onResend={resendCode} />
      </form>
      <div className="py-2">
        <div className="h-px bg-linear-to-r from-background-gradient-mask-transparent via-divider-regular to-background-gradient-mask-transparent"></div>
      </div>
      <div onClick={() => router.back()} className="flex h-9 cursor-pointer items-center justify-center text-text-tertiary">
        <div className="bg-background-default-dimm inline-block rounded-full p-1">
          <RiArrowLeftLine size={12} />
        </div>
        <span className="system-xs-regular ml-2">{t('back', { ns: 'login' })}</span>
      </div>
    </div>
  )
}

```

### web/app/(shareLayout)/webapp-reset-password/layout.tsx
```tsx
'use client'
import Header from '@/app/signin/_header'

import { useGlobalPublicStore } from '@/context/global-public-context'
import { cn } from '@/utils/classnames'

export default function SignInLayout({ children }: any) {
  const { systemFeatures } = useGlobalPublicStore()
  return (
    <>
      <div className={cn('flex min-h-screen w-full justify-center bg-background-default-burn p-6')}>
        <div className={cn('flex w-full shrink-0 flex-col rounded-2xl border border-effects-highlight bg-background-default-subtle')}>
          <Header />
          <div className={
            cn(
              'flex w-full grow flex-col items-center justify-center',
              'px-6',
              'md:px-[108px]',
            )
          }
          >
            <div className="flex w-[400px] flex-col">
              {children}
            </div>
          </div>
          {!systemFeatures.branding.enabled && (
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

### web/app/(shareLayout)/webapp-reset-password/set-password/page.tsx
```tsx
'use client'
import { RiCheckboxCircleFill } from '@remixicon/react'
import { useCountDown } from 'ahooks'
import { useCallback, useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import { validPassword } from '@/config'
import { useRouter, useSearchParams } from '@/next/navigation'
import { changeWebAppPasswordWithToken } from '@/service/common'
import { cn } from '@/utils/classnames'

const ChangePasswordForm = () => {
  const { t } = useTranslation()
  const router = useRouter()
  const searchParams = useSearchParams()
  const token = decodeURIComponent(searchParams.get('token') || '')

  const [password, setPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [showSuccess, setShowSuccess] = useState(false)
  const [showPassword, setShowPassword] = useState(false)
  const [showConfirmPassword, setShowConfirmPassword] = useState(false)

  const showErrorMessage = useCallback((message: string) => {
    toast.error(message)
  }, [])

  const getSignInUrl = () => {
    return `/webapp-signin?redirect_url=${searchParams.get('redirect_url') || ''}`
  }

  const AUTO_REDIRECT_TIME = 5000
  const [leftTime, setLeftTime] = useState<number | undefined>(undefined)
  const [countdown] = useCountDown({
    leftTime,
    onEnd: () => {
      router.replace(getSignInUrl())
    },
  })

  const valid = useCallback(() => {
    if (!password.trim()) {
      showErrorMessage(t('error.passwordEmpty', { ns: 'login' }))
      return false
    }
    if (!validPassword.test(password)) {
      showErrorMessage(t('error.passwordInvalid', { ns: 'login' }))
      return false
    }
    if (password !== confirmPassword) {
      showErrorMessage(t('account.notEqual', { ns: 'common' }))
      return false
    }
    return true
  }, [password, confirmPassword, showErrorMessage, t])

  const handleChangePassword = useCallback(async () => {
    if (!valid())
      return
    try {
      await changeWebAppPasswordWithToken({
        url: '/forgot-password/resets',
        body: {
          token,
          new_password: password,
          password_confirm: confirmPassword,
        },
      })
      setShowSuccess(true)
      setLeftTime(AUTO_REDIRECT_TIME)
    }
    catch (error) {
      console.error(error)
    }
  }, [password, token, valid, confirmPassword])

  return (
    <div className={
      cn(
        'flex w-full grow flex-col items-center justify-center',
        'px-6',
        'md:px-[108px]',
      )
    }
    >
      {!showSuccess && (
        <div className="flex flex-col md:w-[400px]">
          <div className="mx-auto w-full">
            <h2 className="title-4xl-semi-bold text-text-primary">
              {t('changePassword', { ns: 'login' })}
            </h2>
            <p className="body-md-regular mt-2 text-text-secondary">
              {t('changePasswordTip', { ns: 'login' })}
            </p>
          </div>

          <div className="mx-auto mt-6 w-full">
            <div className="bg-white">
              {/* Password */}
              <div className="mb-5">
                <label htmlFor="password" className="system-md-semibold my-2 text-text-secondary">
                  {t('account.newPassword', { ns: 'common' })}
                </label>
                <div className="relative mt-1">
                  <Input
                    id="password"
                    type={showPassword ? 'text' : 'password'}
                    value={password}
                    onChange={e => setPassword(e.target.value)}
                    placeholder={t('passwordPlaceholder', { ns: 'login' }) || ''}
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
                <div className="body-xs-regular mt-1 text-text-secondary">{t('error.passwordInvalid', { ns: 'login' })}</div>
              </div>
              {/* Confirm Password */}
              <div className="mb-5">
                <label htmlFor="confirmPassword" className="system-md-semibold my-2 text-text-secondary">
                  {t('account.confirmPassword', { ns: 'common' })}
                </label>
                <div className="relative mt-1">
                  <Input
                    id="confirmPassword"
                    type={showConfirmPassword ? 'text' : 'password'}
                    value={confirmPassword}
                    onChange={e => setConfirmPassword(e.target.value)}
                    placeholder={t('confirmPasswordPlaceholder', { ns: 'login' }) || ''}
                  />
                  <div className="absolute inset-y-0 right-0 flex items-center">
                    <Button
                      type="button"
                      variant="ghost"
                      onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                    >
                      {showConfirmPassword ? '👀' : '😝'}
                    </Button>
                  </div>
                </div>
              </div>
              <div>
                <Button
                  variant="primary"
                  className="w-full"
                  onClick={handleChangePassword}
                >
                  {t('changePasswordBtn', { ns: 'login' })}
                </Button>
              </div>
            </div>
          </div>
        </div>
      )}
      {showSuccess && (
        <div className="flex flex-col md:w-[400px]">
          <div className="mx-auto w-full">
            <div className="mb-3 flex h-14 w-14 items-center justify-center rounded-2xl border border-components-panel-border-subtle font-bold shadow-lg">
              <RiCheckboxCircleFill className="h-6 w-6 text-text-success" />
            </div>
            <h2 className="title-4xl-semi-bold text-text-primary">
              {t('passwordChangedTip', { ns: 'login' })}
            </h2>
          </div>
          <div className="mx-auto mt-6 w-full">
            <Button
              variant="primary"
              className="w-full"
              onClick={() => {
                setLeftTime(undefined)
                router.replace(getSignInUrl())
              }}
            >
              {t('passwordChanged', { ns: 'login' })}
              {' '}
              (
              {Math.round(countdown / 1000)}
              )
              {' '}
            </Button>
          </div>
        </div>
      )}
    </div>
  )
}

export default ChangePasswordForm

```

### web/app/(shareLayout)/webapp-reset-password/page.tsx
```tsx
'use client'
import { RiArrowLeftLine, RiLockPasswordLine } from '@remixicon/react'
import { noop } from 'es-toolkit/function'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import Button from '@/app/components/base/button'
import Input from '@/app/components/base/input'
import { toast } from '@/app/components/base/ui/toast'
import { COUNT_DOWN_KEY, COUNT_DOWN_TIME_MS } from '@/app/components/signin/countdown'
import { emailRegex } from '@/config'
import { useLocale } from '@/context/i18n'
import useDocumentTitle from '@/hooks/use-document-title'

import Link from '@/next/link'
import { useRouter, useSearchParams } from '@/next/navigation'
import { sendResetPasswordCode } from '@/service/common'

export default function CheckCode() {
  const { t } = useTranslation()
  useDocumentTitle('')
  const searchParams = useSearchParams()
  const router = useRouter()
  const [email, setEmail] = useState('')
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
      const res = await sendResetPasswordCode(email, locale)
      if (res.result === 'success') {
        localStorage.setItem(COUNT_DOWN_KEY, `${COUNT_DOWN_TIME_MS}`)
        const params = new URLSearchParams(searchParams)
        params.set('token', encodeURIComponent(res.data))
        params.set('email', encodeURIComponent(email))
        router.push(`/webapp-reset-password/check-code?${params.toString()}`)
      }
      else if (res.code === 'account_not_found') {
        toast.error(t('error.registrationNotAllowed', { ns: 'login' }))
      }
      else {
        toast.error(res.data)
      }
    }
    catch (error) {
      console.error(error)
    }
    finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="flex flex-col gap-3">
      <div className="inline-flex h-14 w-14 items-center justify-center rounded-2xl border border-components-panel-border-subtle bg-background-default-dodge shadow-lg">
        <RiLockPasswordLine className="h-6 w-6 text-2xl text-text-accent-light-mode-only" />
      </div>
      <div className="pb-4 pt-2">
        <h2 className="title-4xl-semi-bold text-text-primary">{t('resetPassword', { ns: 'login' })}</h2>
        <p className="body-md-regular mt-2 text-text-secondary">
          {t('resetPasswordDesc', { ns: 'login' })}
        </p>
      </div>

      <form onSubmit={noop}>
        <input type="text" className="hidden" />
        <div className="mb-2">
          <label htmlFor="email" className="system-md-semibold my-2 text-text-secondary">{t('email', { ns: 'login' })}</label>
          <div className="mt-1">
            <Input id="email" type="email" disabled={loading} value={email} placeholder={t('emailPlaceholder', { ns: 'login' }) as string} onChange={e => setEmail(e.target.value)} />
          </div>
          <div className="mt-3">
            <Button loading={loading} disabled={loading} variant="primary" className="w-full" onClick={handleGetEMailVerificationCode}>{t('sendVerificationCode', { ns: 'login' })}</Button>
          </div>
        </div>
      </form>
      <div className="py-2">
        <div className="h-px bg-linear-to-r from-background-gradient-mask-transparent via-divider-regular to-background-gradient-mask-transparent"></div>
      </div>
      <Link href={`/webapp-signin?${searchParams.toString()}`} className="flex h-9 items-center justify-center text-text-tertiary hover:text-text-primary">
        <div className="inline-block rounded-full bg-background-default-dimmed p-1">
          <RiArrowLeftLine size={12} />
        </div>
        <span className="system-xs-regular ml-2">{t('backToLogin', { ns: 'login' })}</span>
      </Link>
    </div>
  )
}

```

### api/controllers/service_api/end_user/error.py
```py
from libs.exception import BaseHTTPException


class EndUserNotFoundError(BaseHTTPException):
    error_code = "end_user_not_found"
    description = "End user not found."
    code = 404

```

### api/controllers/service_api/end_user/__init__.py
```py
from . import end_user

__all__ = ["end_user"]

```

### api/controllers/service_api/end_user/end_user.py
```py
from uuid import UUID

from flask_restx import Resource

from controllers.service_api import service_api_ns
from controllers.service_api.end_user.error import EndUserNotFoundError
from controllers.service_api.wraps import validate_app_token
from fields.end_user_fields import EndUserDetail
from models.model import App
from services.end_user_service import EndUserService


@service_api_ns.route("/end-users/<uuid:end_user_id>")
class EndUserApi(Resource):
    """Resource for retrieving end user details by ID."""

    @service_api_ns.doc("get_end_user")
    @service_api_ns.doc(description="Get an end user by ID")
    @service_api_ns.doc(
        params={"end_user_id": "End user ID"},
        responses={
            200: "End user retrieved successfully",
            401: "Unauthorized - invalid API token",
            404: "End user not found",
        },
    )
    @validate_app_token
    def get(self, app_model: App, end_user_id: UUID):
        """Get end user detail.

        This endpoint is scoped to the current app token's tenant/app to prevent
        cross-tenant/app access when an end-user ID is known.
        """

        end_user = EndUserService.get_end_user_by_id(
            tenant_id=app_model.tenant_id, app_id=app_model.id, end_user_id=str(end_user_id)
        )
        if end_user is None:
            raise EndUserNotFoundError()

        return EndUserDetail.model_validate(end_user).model_dump(mode="json")

```

## Output Instructions

Output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-024.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
