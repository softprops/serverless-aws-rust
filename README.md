# serverless AWS Rust template

A sample template for bootstrapping [Rustlang AWS Lambda](https://github.com/awslabs/aws-lambda-rust-runtime/) applications with ⚡ serverless framework ⚡.

## ✨ features

* 🦀 Build Rustlang applications targeting AWS Lambda with ease
* 🛵 Continuous integration testing with GitHub Actions
* 🚀 Continuous deployment with GitHub Actions
* 🧪 Getting started unit tests

## 📦 install

Install the [serverless framework](https://www.serverless.com/framework/docs/getting-started/) cli.

Then then run the following in your terminal

```bash
$ npx serverless install \
  --url https://github.com/softprops/serverless-aws-rust \
  --name my-new-app
```

This will download the source of a sample Rustlang application and unpack it as a new service named
"my-new-app" in a directory called "my-new-app"


## 🧙 how to be a wizard

Assuming you have [aws credentials with appropriate deployment permissions configured](https://serverless.com/framework/docs/providers/aws/guide/credentials/) (if you already use any existing AWS tooling installed you likely already have this configured), you can impress your friends by creating a project that is _born_ in a production environment.

```bash
$ npx serverless install \
  --url https://github.com/softprops/serverless-aws-rust \
  --name my-new-app \
  && cd my-new-app \
  && npm ci \
  && npx serverless deploy
```

`npm ci` will make sure npm dependencies are installed based directly on your package-lock.json file. This only needs run once.
The first time you run `npx serverless deploy` this project will pull down and compile the base set
of dependencies and your application. Unless the dependencies change afterwards,
this should only happen once, resulting in an out of the box rapid deployment
cycle.

## 🛵 continuous integration and deployment

This template includes an example [GitHub actions](https://travis-ci.org/) [configuration file](.github/workflows/main.yml) which can unlock a virtuous cycle of continuous integration and deployment
( i.e all tests are run on prs and every push to master results in a deployment ).

GitHub actions is managed simply by the presence of a file checked into your repository. To set up GitHub Actions to deploy to AWS you'll need to do a few things

Firstly, version control your source. [Github](https://github.com/) is free for opensource.

```bash
$ git init
$ git remote add origin git@github.com:{username}/{my-new-service}.git
```

Store a `AWS_ACCESS_KEY_ID` `AWS_SECRET_ACCESS_KEY` used for aws deployment in your repositories secrets https://github.com/{username}/{my-new-service}/settings/secrets

Add your changes to git and push them to GitHub.

Finally, open https://github.com/{username}/{my-new-service}/actions in your browser and grab a bucket of popcorn 🍿

## 🔫 function triggering

With your function deployed you can now start triggering it using `serverless` framework directly or
the AWS integration you've configured to trigger it on your behalf

```bash
$ npx serverless invoke -f hello -d '{"foo":"bar"}'
```

## 🔬 logs

With your function deployed you can now tail
it's logs right from your project

```bash
$ npx serverless logs -f hello
```

## 👴 retiring

Good code should be easily replaceable. Good code should also be easily disposable. Retiring applications should be as easy as creating and deploying them them. The dual of `serverless deploy` is `serverless remove`. Use this for retiring services and cleaning up resources.

```bash
$ npx serverless remove
```

## ℹ️  additional information

* See the [serverless-rust plugin's documentation](https://github.com/softprops/serverless-rust) for more information on plugin usage.

* See the [aws rust runtime's documentation](https://github.com/awslabs/aws-lambda-rust-runtime) for more information on writing Rustlang lambda functions

## 👯 contributing

This template's intent is to set a minimal baseline for getting engineers up an running with a set of repeatable best practices. See something you'd like in this template that would help others? Feel free to [open a new GitHub issue](https://github.com/softprops/serverless-aws-rust/issues/new). Pull requests are also welcome.