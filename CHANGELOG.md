# Changelog

## [1.0.0](https://github.com/jzeuzs/legion/compare/v0.1.3...v1.0.0) (2024-07-15)


### ⚠ BREAKING CHANGES

* rewrite http server to axum ([#71](https://github.com/jzeuzs/legion/issues/71))
* remove cache ([#70](https://github.com/jzeuzs/legion/issues/70))

### Features

* **api:** add logging ([#76](https://github.com/jzeuzs/legion/issues/76)) ([ef44842](https://github.com/jzeuzs/legion/commit/ef44842522d36b5cea6be9e6704829a459357601))
* **docs:** add api documentation ([#75](https://github.com/jzeuzs/legion/issues/75)) ([e83329d](https://github.com/jzeuzs/legion/commit/e83329d1cb22e6691424c87dc1e95592fcc1ceb7))
* **eval:** add input tests ([#72](https://github.com/jzeuzs/legion/issues/72)) ([dc88921](https://github.com/jzeuzs/legion/commit/dc88921b3fcbd7030e4a9afc90021d3785f1906a))
* remove cache ([#70](https://github.com/jzeuzs/legion/issues/70)) ([439358c](https://github.com/jzeuzs/legion/commit/439358c6ddb6c84e670d74f0dcff503e9257848c))
* rewrite http server to axum ([#71](https://github.com/jzeuzs/legion/issues/71)) ([8010af9](https://github.com/jzeuzs/legion/commit/8010af95f810eb33f8b8071ab9b18228b63547dd))


### Bug Fixes

* **deps:** update all non-major dependencies ([#35](https://github.com/jzeuzs/legion/issues/35)) ([705cec5](https://github.com/jzeuzs/legion/commit/705cec53bf008c6fcfa67d2bda23ece8865dc33e))
* **deps:** update all non-major dependencies ([#45](https://github.com/jzeuzs/legion/issues/45)) ([f5694c9](https://github.com/jzeuzs/legion/commit/f5694c9435dac622c67d1e2d75016423c1bd8bc7))
* **deps:** update all non-major dependencies ([#60](https://github.com/jzeuzs/legion/issues/60)) ([9f8ba9b](https://github.com/jzeuzs/legion/commit/9f8ba9b35ebbeeaff4863ba8759f743c05cb4a21))
* **deps:** update rust crate libc to 0.2.137 ([#40](https://github.com/jzeuzs/legion/issues/40)) ([6a95231](https://github.com/jzeuzs/legion/commit/6a95231ef2996d4643cabcaa28a557e1e7cf9eed))
* **deps:** update rust crate moka to 0.9.5 ([#43](https://github.com/jzeuzs/legion/issues/43)) ([7f45b39](https://github.com/jzeuzs/legion/commit/7f45b3984911a13a911daecfbeddf541c2773445))
* **deps:** update rust crate owo-colors to v4 ([#69](https://github.com/jzeuzs/legion/issues/69)) ([0849901](https://github.com/jzeuzs/legion/commit/0849901f9e4744da96024e15c1c481249d535f33))
* **workflow:** move to new release please action ([#53](https://github.com/jzeuzs/legion/issues/53)) ([53d5a9d](https://github.com/jzeuzs/legion/commit/53d5a9dea019fa0689e6856f4babb3b4d13155c1))

## [0.1.3](https://github.com/jzeuzs/legion/compare/v0.1.2...v0.1.3) (2022-10-16)


### 🐛 Bug Fixes

* **ci:** `website-screenshot` -> `legion` ([7a529a7](https://github.com/jzeuzs/legion/commit/7a529a79562f3f821491cb7af6d99d27d60a086b))

## [0.1.2](https://github.com/jzeuzs/legion/compare/v0.1.1...v0.1.2) (2022-10-16)


### 🐛 Bug Fixes

* **ci:** `website-screenshot` -> `legion` ([ecb9acb](https://github.com/jzeuzs/legion/commit/ecb9acb70a951531f1d972b5a8045d821e0a432f))

## [0.1.1](https://github.com/jzeuzs/legion/compare/v0.1.0...v0.1.1) (2022-10-16)


### 🐛 Bug Fixes

* **ci:** fix release workflow ([55e253c](https://github.com/jzeuzs/legion/commit/55e253c8b768d53e610a29c6d96d7a79bb547f49))

## 0.1.0 (2022-10-15)


### 🏠 Refactor

* change `Default` impl of `Config` ([6fd2266](https://github.com/jzeuzs/legion/commit/6fd22663c05979beed3337747fc1a810ca69feb3))
* change `Default` impl of `Config` ([3dc4db0](https://github.com/jzeuzs/legion/commit/3dc4db0e475140030f71d2645faf3f553a46a76a))
* only use one `format!(...)` syntax for readability ([84f2079](https://github.com/jzeuzs/legion/commit/84f20792a7db1eed996566ea33cd5ee98be9259e))
* only use one `format!(...)` syntax for readability ([cbee25b](https://github.com/jzeuzs/legion/commit/cbee25b29db6ae89bc21603d4c43e4b786345cfc))


### 🐛 Bug Fixes

* **deps:** update all non-major dependencies ([#16](https://github.com/jzeuzs/legion/issues/16)) ([9680a0f](https://github.com/jzeuzs/legion/commit/9680a0f63170a76322d0a6c9971c40bf9e86e6da))
* **deps:** update rust crate moka to 0.9.0 ([#8](https://github.com/jzeuzs/legion/issues/8)) ([10861a3](https://github.com/jzeuzs/legion/commit/10861a38820b86ba972a6e694d02ada7b81a31a5))
* resolve `RUSTSEC-2020-0071` ([f1f63b6](https://github.com/jzeuzs/legion/commit/f1f63b644fc75c833ca067f9fb7874e192061478))


### 🚀 Features

* add bun ([#15](https://github.com/jzeuzs/legion/issues/15)) ([4fdd293](https://github.com/jzeuzs/legion/commit/4fdd29362c2415674572f20ca238a858a1988095))
* add zig ([#14](https://github.com/jzeuzs/legion/issues/14)) ([acbfba4](https://github.com/jzeuzs/legion/commit/acbfba46d9c7de52debda76f92747cebfb2f94ee))
* **assembly:** add more assemblers ([#13](https://github.com/jzeuzs/legion/issues/13)) ([6216c20](https://github.com/jzeuzs/legion/commit/6216c204b492d4a49ebe45ecd83cbb881e49aaab))
* **ci:** setup bors ([f6e7cd7](https://github.com/jzeuzs/legion/commit/f6e7cd7672d02cebedb9c8c31938df176038b6bc))
* **docker:** add docker image ([#23](https://github.com/jzeuzs/legion/issues/23)) ([ae1da5e](https://github.com/jzeuzs/legion/commit/ae1da5e51abfcdb2379f27d94075838efea9ab85))
* implement sqlite ([#11](https://github.com/jzeuzs/legion/issues/11)) ([8911fc9](https://github.com/jzeuzs/legion/commit/8911fc93173a8f1bb485b946db439f7b2161505e))
* init project ([2c5b5cd](https://github.com/jzeuzs/legion/commit/2c5b5cd668721ffd92e985081a4179958bd62743))
* init setup ([bbc9380](https://github.com/jzeuzs/legion/commit/bbc93803c2315769d7286ee4d22721e56891b64d))
* **languages:** add Objective-C ([#22](https://github.com/jzeuzs/legion/issues/22)) ([187a0ad](https://github.com/jzeuzs/legion/commit/187a0ad8c72652eaddd4e3246f8bc8199ca5a9de))


### 📝 Documentation

* add example config ([6cc43fb](https://github.com/jzeuzs/legion/commit/6cc43fb93183a6794b42b61f2f499ff4f38dc14b))
* add example config ([8c09e45](https://github.com/jzeuzs/legion/commit/8c09e45b0f221bd6d697666c7a7f9b896b32b241))
* add more info on readme ([#26](https://github.com/jzeuzs/legion/issues/26)) ([1a8f3ce](https://github.com/jzeuzs/legion/commit/1a8f3ce3f0dd8c4ba6480066ffca67450a8280b6))
* add openapi spec ([#25](https://github.com/jzeuzs/legion/issues/25)) ([3802181](https://github.com/jzeuzs/legion/commit/38021818a53e5e12759db727a8fa3a07b78fde6c))
