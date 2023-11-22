## git-gpt

Use GPT 3.5 API generate git commit log.

### Install
```bash
cargo install git-gpt
```

### Usage
```bash
export OPEAN_AI=YOUR_TOKEN
# Token is here https://platform.openai.com/account/api-keys
git gpt
git push
```
or you can use proxy url, take a look https://github.com/buhe/openai_aws_proxy, proxy serve on aws serverless, proxy used by open ai block regsion.
```bash
export PROXY_URL=YOUR_PROXY_URL
```

### Tasks

- [x] repo
- [x] add all
- [x] commit
- [x] diff file
- [x] generate git commit log
- [x] proxy url
- [x] skip some files.
