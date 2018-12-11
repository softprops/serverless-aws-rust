# serverless AWS Rust template

A sample template for bootstraping [Rustlang AWS Lambda](https://github.com/awslabs/aws-lambda-rust-runtime/) applications with ⚡ serverless framework ⚡.

> ℹ️ See the [serverless-rust plugin's documentation](https://github.com/softprops/serverless-rust) for more information on plugin usage.

> ℹ️ See the [aws rust runtime's documentation](https://github.com/awslabs/aws-lambda-rust-runtime) for more information on writing Rustlang lambda functions

## ✨ features

* 🦀 Build Rustlang applications with ease
* 🛵 Continuous integration testing with travis CI
* 🚀 Continuous deployment with travis CI
* 👩‍🏭 Simplified make based workflow

## 📦 install

Install the [serverless framework](https://serverless.com/framework/) cli.

Then then run the following in your terminal

```bash
$ serverless install \
  --url https://github.com/softprops/serverless-aws-rust \
  --name my-new-app
```

This will download the source of a sample crowbar application and unpack it as a new service named
"my-new-app" in a directory called "my-new-app"


## 🧙 how to be a wizard

Assumming you have aws credentials with appropriate deployment permissions configured
in a profile named "prod", you could impress your friends by creating a project
that is _born_ in production.

```bash
$ serverless install \
  --url https://github.com/softprops/serverless-aws-rust \
  --name my-new-app \
  && cd my-new-app \
  && AWS_PROFILE=prod make dependencies deploy
```

`make dependencies` will make sure npm dependencies are installed, this only needs run once.
The first time you run `make deploy` it will pull down and compile the base set
of dependencies and your application. Unless the dependencies change afterwards,
this should only happen once, resulting in an out of the box rapid deployment
cycle.

## 🛵 continuous integration and deployment

This template includes an example [travis](https://travis-ci.org/) [configuration file](.travis.yml) which can unlock a virtuous cycle of continuous integration and deployment
( i.e all tests are run on prs and every push to master results in a deployment ).

To set up travis you will need to do a view things.

Firstly, version control your source. [Github](https://github.com/) is free for opensource.

```bash
$ git init
$ git remote add origin git@github.com:{username}/{my-new-service}.git
```

Using the [travis cli](https://github.com/travis-ci/travis.rb#installation),
 bootstrap your git repos' travis integration.

```bash
$ travis enable
# set up AWS credentials for serverless deployment
# https://serverless.com/framework/docs/providers/aws/guide/credentials/
$ travis env set AWS_ACCESS_KEY_ID 'xxx'
$ travis env set AWS_SECRET_ACCESS_KEY 'xxx'
```

> ⭐ You can optionally generate code coverage reports with [coveralls](http://coveralls.io/) by enabling your repo [here](https://coveralls.io/repos/new). You may need to sync repos first. You can then view your coverage reports at https://coveralls.io/github/{username}/{my-new-service}

Add your changes to git and push them to github.

Finally, https://travis-ci.org/{username}/{my-new-service} in your browser and grab a bucket of popcorn 🍿

## 🔫 function triggering

With your function deployed in production you can now start triggering it using `serverless` framework directly or
the AWS integration you've configured to trigger it on your behalf

```sh
$ AWS_PROFILE=prod npx serverless invoke --stage prod -f hello -d '{"foo":"bar"}'
```

## 👴 retiring

Experimentation will likely facilitate retiring ideas. Retiring applications should be as easy as creating and deploying them them. This project provides
 a dual to `make deploy` for doing so: `make destroy`

```bash
$ AWS_PROFILE=prod make destroy
```