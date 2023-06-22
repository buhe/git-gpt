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
or you can use proxy url, see https://github.com/justjavac/openai-proxy, proxy serve on deno edge function at USA, proxy used by open ai block regsion.
```bash
export PROXY_URL=YOUR_PROXY_URL
```

### Tasks

- [x] repo
- [x] add all
- [x] commit
- [x] request gpt
- [x] diff
- [x] generate git commit log
- [x] gpt log
- [x] use git cong instead of hard code
- [x] update document
- [x] proxy url
- [x] skip .vscode and *.lock
