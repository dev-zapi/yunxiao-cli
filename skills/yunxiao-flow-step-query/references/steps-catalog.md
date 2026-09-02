# Yunxiao Flow Steps Snapshot

Source: [official Steps list](https://help.aliyun.com/zh/yunxiao/user-guide/step-steps-list)
Snapshot date: 2026-09-02
Entries: 62

This is an offline fallback. The skill's query script fetches the official page
by default; use the linked detail document to confirm each Step's current
`with` parameters before writing a pipeline.

### 上传

`OSSUpload`, `ReportUpload`, `ArtifactUpload`, `SingleArtifactUpload`

### 下载

`OSSDownload`

### 代码

`AddGitTag`, `MergeBranch`, `DeleteBranch`, `CheckBranchBehind`, `GetGitMessage`

### 发布

`HelmRelease`, `KubectlApply`, `KubectlSetImage`, `ROSDeploy`, `AppStackFlowDeploy`, `FCDeploy`

### 工具

`EcsTagSwitch`, `OSSDelete`, `ServerlessDevs`, `Command`, `ReplaceVariables`, `SetVariables`

### 构建

`AspNetBuild`, `HelmPush`, `GccBuild`, `GolangBuild`, `JavaBuild`, `DotNetCoreBuild`, `NodeBuild`, `PhpBuild`, `PythonBuild`, `RubyBuild`, `RustBuild`, `CustomEnvironmentBuild`, `PrivateRegistryDockerBuild`, `ACRDockerBuild`, `ACREEDockerBuild`

### 测试

`AndroidUnitTest`, `GolangUnitTest`, `GradleUnitTest`, `JunitReport`, `MavenUnitTest`, `NodeUnitTest`, `PhpCodeceptionUnitTest`, `PhpUnitTest`, `PythonUnitTest`

### 覆盖率

`Cobertura`, `JaCoCo`, `PythonTestCoverage`

### 静态扫描

`AndroidCodeScan`, `CppCodeScan`, `GolangCodeScan`, `JavaP3CScan`, `JavaFindBugs`, `JavaScriptCodeScan`, `PhpMetricsScan`, `PythonBandit`, `PythonCodeScan`, `PythonDependencySecurityScan`, `SonarQube`, `TSLint`, `Pinpoint`
