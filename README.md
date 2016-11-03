# Description

This tool will find urls in stdin that match the "protected" urls by urldefense
from proofpoint, and replace them with the original url.

# mutt

To use with mutt, add the following configuration line:

```
macro attach,compose,index,pager <F6> ":set pipe_decode=yes\n<pipe-message> ~/bin/url-cleaner -c\
     | less -R\n:set pipe_decode=no\n" "call urlcleaner to clean URLs from a message"
```
