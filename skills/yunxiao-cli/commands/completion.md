# Shell 补全命令手册

`yunxiao completion` 命令用于生成 Shell 补全脚本。

---

## 命令列表

| 命令 | 说明 |
|------|------|
| `yunxiao completion generate` | 生成 Shell 补全脚本 |

---

## 生成 Shell 补全

### 基本用法

```bash
yunxiao completion generate --shell <shell>
```

### 参数

| 参数 | 说明 | 必需 |
|------|------|------|
| `--shell` | Shell 类型（bash/zsh/powershell） | 是 |

### 支持的 Shell

- `bash` - Bash Shell
- `zsh` - Zsh Shell
- `powershell` - PowerShell

---

## Bash 补全

### 生成补全脚本

```bash
yunxiao completion generate --shell bash > ~/.local/share/bash-completion/completions/yunxiao
```

### 启用补全

```bash
# 加载补全
source ~/.bashrc
```

### 补全文件位置

- 系统级: `/usr/share/bash-completion/completions/yunxiao`
- 用户级: `~/.local/share/bash-completion/completions/yunxiao`

---

## Zsh 补全

### 生成补全脚本

```bash
yunxiao completion generate --shell zsh > ~/.zfunc/_yunxiao
```

### 启用补全

在 `~/.zshrc` 中添加：

```bash
fpath+=~/.zfunc
autoload -U compinit && compinit
```

然后：

```bash
source ~/.zshrc
```

---

## PowerShell 补全

### 生成补全脚本

```powershell
yunxiao completion generate --shell powershell > yunxiao.ps1
```

### 启用补全

```powershell
# 在 PowerShell profile 中添加
. yunxiao.ps1

# 或临时加载
. ./yunxiao.ps1
```

---

## 常见用法

### 快速启用 Bash 补全

```bash
# 一条命令完成
yunxiao completion generate --shell bash > ~/.local/share/bash-completion/completions/yunxiao && source ~/.bashrc
```

### 快速启用 Zsh 补全

```bash
# 生成补全文件
yunxiao completion generate --shell zsh > ~/.zfunc/_yunxiao

# 编辑 ~/.zshrc 添加 fpath
echo 'fpath+=~/.zfunc' >> ~/.zshrc

# 重新加载
source ~/.zshrc
```

---

## 补全功能

启用补全后，可以：

- 命令自动补全：输入 `yunxiao ` 按 Tab
- 子命令补全：输入 `yunxiao projex ` 按 Tab
- 参数补全：输入 `yunxiao projex projects search --` 按 Tab

---

## 故障排查

### 补全不生效

**Bash**:
```bash
# 检查补全文件
ls ~/.local/share/bash-completion/completions/yunxiao

# 手动加载
source ~/.local/share/bash-completion/completions/yunxiao
```

**Zsh**:
```bash
# 检查补全文件
ls ~/.zfunc/_yunxiao

# 检查 fpath
echo $fpath

# 手动加载
autoload -U _yunxiao && compdef _yunxiao yunxiao
```

### 补全文件位置错误

**原因**: 补全文件路径不正确

**解决方案**:
```bash
# Bash - 确认系统补全目录
ls /usr/share/bash-completion/completions/

# Zsh - 确认补全目录
echo $fpath
```