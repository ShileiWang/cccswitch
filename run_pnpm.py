import os
import subprocess
import sys

env = dict(os.environ)
# 本机直连被拒，npm 流量必须走本地代理（HTTP_PROXY 指向端口，7897 已失效）。
proxy = os.environ.get("HTTP_PROXY") or "http://127.0.0.1:24863"
env["HTTP_PROXY"] = proxy
env["HTTPS_PROXY"] = proxy
env["http_proxy"] = proxy
env["https_proxy"] = proxy
env["npm_config_proxy"] = proxy
env["npm_config_https_proxy"] = proxy
env["npm_config_registry"] = "https://registry.npmmirror.com/"
env["npm_config_manage_package_manager_versions"] = "false"
env["COREPACK_ENABLE_STRICT"] = "0"

exe = r"C:\Users\Admin\AppData\Roaming\npm\node_modules\pnpm\bin\pnpm.mjs"
args = sys.argv[1:] or ["install"]
result = subprocess.run(
    ["node", exe] + args,
    capture_output=True,
    text=True,
    encoding="utf-8",
    errors="replace",
    env=env,
)
print("RC", result.returncode)
print(result.stdout[-6000:])
print("ERR", result.stderr[-3000:])
