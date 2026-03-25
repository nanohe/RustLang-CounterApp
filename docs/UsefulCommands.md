# Useful Commands

## Configure Local Git Identity

Set the Git user name and email for this repository only:

```bash
git config --local user.name 'NanoHE' && git config --local user.email 'nano.he@qq.com'
```

## Verify Local Git Identity

Check the values stored in the repository-local Git config:

```bash
git config --local --get-regexp '^user\.(name|email)$'
```


## Use this workaround for Git:
```bash
env -u http_proxy -u https_proxy -u HTTP_PROXY -u HTTPS_PROXY -u ALL_PROXY -u all_proxy git fetch origin
```
This command temporarily unsets the proxy environment variables for the `git fetch` command, allowing it to bypass any misconfigured proxies that may be causing connectivity issues with the remote repository.

```bash
git add .
git commit -m "your message"
env -u http_proxy -u https_proxy -u HTTP_PROXY -u HTTPS_PROXY -u ALL_PROXY -u all_proxy git push -u origin master
```

This command sequence stages all changes, commits them with a message, and then pushes to the `master` branch on the remote repository while temporarily unsetting the proxy environment variables to avoid connectivity issues.