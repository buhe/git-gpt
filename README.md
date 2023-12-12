## git-gpt

Use GPT 3.5 API generate git commit log.

### Install
```bash
cargo install git-gpt
```

### Usage
```bash
export OPENAI_API_KEY=YOUR_TOKEN
# Token is here https://platform.openai.com/account/api-keys
git gpt
git push
```
or you can use proxy url, take a look https://github.com/buhe/openai_aws_proxy, proxy serve on aws serverless, proxy used by open ai block regsion.
```bash
export OPENAI_URL=YOUR_PROXY_URL
```

### Task

- [x] Add all files
- [x] Commit all
- [x] Diff files and skip some files
- [x] Generate git commit log
- [x] Add proxy
