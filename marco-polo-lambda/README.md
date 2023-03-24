### Steps to run

* `make format` to format code
* `make lint` to lint code
* `make release-arm` to build release binary which is `cargo lambda build --release --arm64`
* `make deploy` to deploy to AWS Lambda which is `cargo lambda deploy`
* `make invoke` 
```rust
(.venv) @yifu-deng ➜ /workspaces/yifu_Weekly-miniproject/marco-polo-lambda (main) $ make invoke
cargo lambda invoke --remote \
                --data-ascii '{"name": "Marco"}' \
                --output-format json \
                marco-polo-lambda
{
  "msg": "Marco says Polo",
  "req_id": "7fe2466e-f6ab-40f6-9e70-17b40f479f3c"
}
```


<<<<<<< HEAD
cause the error of -` × failed to assume new lambda role. Try deploying using the flag --iam-role arn:aws:iam::969594881030:role/cargo-lambda-role-08eb8c18-cb05-45a5-ba02-0d46a54eb8e5` 

I revise the `Makefile` to `make deploy` to `cargo lambda deploy --iam-role arn:aws:iam::969594881030:role/cargo-lambda-role-08eb8c18-cb05-45a5-ba02-0d46a54eb8e5` and it works.
=======
cause the error of -` × failed to assume new lambda role. Try deploying using the flag --iam-role...` 
I revise the `Makefile` to `make deploy` to `cargo lambda deploy --iam-role...` and it works.
>>>>>>> 83920c4 (revise README)

