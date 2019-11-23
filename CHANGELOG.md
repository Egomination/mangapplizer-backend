<a name="0.3.0"></a>
## 0.3.0 (2019-11-23)


#### Style

*   rustfmt ([4abc29a4](https://github.com/Egomination/mangapplizer-backend/commit/4abc29a4447524a5614ab5f4b5dccd303eb38be9))
*   apply rustfmt on schema.rs ([590ff98b](https://github.com/Egomination/mangapplizer-backend/commit/590ff98bfacddc82e2e407366eb609175dc37771))
*   add rustfmt config ([d7e09e03](https://github.com/Egomination/mangapplizer-backend/commit/d7e09e03147c5a42bf3bd1e484c1e64505f0f0a2))
*   formatting ([8f178bdb](https://github.com/Egomination/mangapplizer-backend/commit/8f178bdb67378d8756ae44e9031bce7f9a3dd3a5))

#### Fix

*   fix staff's insertion conflict ([d65c9c2f](https://github.com/Egomination/mangapplizer-backend/commit/d65c9c2f57faddb6e2ce28787f7f7c710c460c6e))
*   make relation's banner_image nullable ([5800b50e](https://github.com/Egomination/mangapplizer-backend/commit/5800b50eb454486ae50ce665f0103d969cb5c5b0))
*   genres rust bindings ([d4a181da](https://github.com/Egomination/mangapplizer-backend/commit/d4a181da437239746a8e2c0bd7d72d60805ab76d))
*   typo change genre_lists to genres_lists ([7f2eeeec](https://github.com/Egomination/mangapplizer-backend/commit/7f2eeeec5c571bb339a1d89c3f359a70a9198627))
*   change relation banner image to nullable ([931aab84](https://github.com/Egomination/mangapplizer-backend/commit/931aab84304f7810070f860b8b659dd5e7ac9000))
*   add missing fields into the manga struct ([3b690135](https://github.com/Egomination/mangapplizer-backend/commit/3b6901356daca7bc567ab4860ec7145640f313b8))
*   add new traits to mangas, series and staffs ([bb6875d6](https://github.com/Egomination/mangapplizer-backend/commit/bb6875d68d0b0c4a612d10cf5e6f8ed928b7de69))
*   generate schema.rs with new fields of the mangas ([d500eb40](https://github.com/Egomination/mangapplizer-backend/commit/d500eb40371b056c34aabdaa30dc6d3b53cfd808))
*   add missing fields of the mangas table ([01d102b3](https://github.com/Egomination/mangapplizer-backend/commit/01d102b34d0ce64cd04a1e912816d01e950156f0))
*   unwrap panic when end_date is null ([4e2a24d7](https://github.com/Egomination/mangapplizer-backend/commit/4e2a24d79ffd7f51f55fb670fbdd7ef4c72d667b))
*   fix unused import warning ([4e44b3c8](https://github.com/Egomination/mangapplizer-backend/commit/4e44b3c88fd509204f934a99fee85fa98bc225f5))
*   remove travis conf ([46e97838](https://github.com/Egomination/mangapplizer-backend/commit/46e978384a064fe364c119d2cba72cb1ad9d6dd4))
*   database func ([3f158734](https://github.com/Egomination/mangapplizer-backend/commit/3f158734410ce8d39dedd13e52a33ce8844d8aff))
* **parser:**
  *  remove chapter log ([8f8acdf6](https://github.com/Egomination/mangapplizer-backend/commit/8f8acdf61c1d54ecb7968cb89b41742f5e19cb22))
  *  remove unnecessary calls in main ([8e3eace6](https://github.com/Egomination/mangapplizer-backend/commit/8e3eace6ee5ece437f3d73b6b1608b529d2f10c1))
  *  fix forgotten function param. ([30afa702](https://github.com/Egomination/mangapplizer-backend/commit/30afa702b5df9b13d69d10e22123e38423aac8e4))

#### Documents

* **changelog:**
  *  update changelog ([3947210f](https://github.com/Egomination/mangapplizer-backend/commit/3947210feda98b19c29115fde29f4c9c96a112c5))
  *  add changelog ([5008d39f](https://github.com/Egomination/mangapplizer-backend/commit/5008d39ff926a5dfe8034c2902c0ec1ce0a3c969))
* **readme:**  add running insturctions on readme ([1689d332](https://github.com/Egomination/mangapplizer-backend/commit/1689d332f628ebad85ee4b5c6c4ea4291f9830a5))

#### Test

* **anilist:**
  *  fix postpayload's one of the test cases ([8ee7e0f1](https://github.com/Egomination/mangapplizer-backend/commit/8ee7e0f18f3e8cdfae4d5ff90f70a50306056305))
  *  add tests ([d93ace90](https://github.com/Egomination/mangapplizer-backend/commit/d93ace9015cf0f3dce328f4085fb4105f503b7f4))
* **core:**  add test for handler logic ([b4aee385](https://github.com/Egomination/mangapplizer-backend/commit/b4aee385de00e0e46d97748f0b958db151160492))

#### CICD

* **travis:**  add script section for running tests ([e2d8894b](https://github.com/Egomination/mangapplizer-backend/commit/e2d8894b38c61582dbd949c19c8d6b84fcfc1e50))

#### Refactor

*   rename k(key) to s(series) as closure variable ([65329f1e](https://github.com/Egomination/mangapplizer-backend/commit/65329f1e7942e7cdf0c99dc5920c688806bf43c6))
*   change unwrap_or_else to unwrap_or ([adb76639](https://github.com/Egomination/mangapplizer-backend/commit/adb76639abf9cf96a31c6881934afec303abea52))
*   rename staff role and name fields of staff ([bb05dc9b](https://github.com/Egomination/mangapplizer-backend/commit/bb05dc9bb1342dcaea7abf1bfe1bfa1ce9c80fc0))
*   seperate create stuff function ([b042e992](https://github.com/Egomination/mangapplizer-backend/commit/b042e9924695afd08572b2caf9bfa016ab0f69d4))
*   create models folder ([c87afbd1](https://github.com/Egomination/mangapplizer-backend/commit/c87afbd1f2a6c4e95b463465b40cd05b634a01e2))
*   remove go ([24529762](https://github.com/Egomination/mangapplizer-backend/commit/24529762fa42d2a2d84572fb1388c7179405d90b))
* **anilist:**  anilist instance creation ([0a0a9eed](https://github.com/Egomination/mangapplizer-backend/commit/0a0a9eed77a752a27c38277baea1a28b91ce2b3d))
* **core:**  serve api ([e82afe52](https://github.com/Egomination/mangapplizer-backend/commit/e82afe52042b197cf8b38d9cc89abdbb81b2b453))
* **docker:**  export environment variables ([2f15d088](https://github.com/Egomination/mangapplizer-backend/commit/2f15d0885f426d6cbee21d0e00a8a4ca5b37b473))
* **parser:**
  *  make parser external dependency ([d28734a0](https://github.com/Egomination/mangapplizer-backend/commit/d28734a073a34d7a483b7488db8af8cbc1782fb3))
  *  put mangarock models in their own file ([402683c2](https://github.com/Egomination/mangapplizer-backend/commit/402683c2dca034f6842e784648659a75e0bc75bf))

#### Feature

*   add genres into the response struct ([e7cc0dcb](https://github.com/Egomination/mangapplizer-backend/commit/e7cc0dcb54ce23b9fad32219be79a9f164f39a82))
*   add genres ([a38bbffa](https://github.com/Egomination/mangapplizer-backend/commit/a38bbffa5ca14c6177166974fe5cfa8014294b3d))
*   add tags ([346b9058](https://github.com/Egomination/mangapplizer-backend/commit/346b9058b718041f7eb2d940ed90cd9a97c79e04))
*   add tags in example input ([9eea77d5](https://github.com/Egomination/mangapplizer-backend/commit/9eea77d54c84932c17ef45535381b7386e1a1e2f))
*   add tags and genres rust bindings ([a74c4d7c](https://github.com/Egomination/mangapplizer-backend/commit/a74c4d7ce7131e5091e9e9bda5abbaaba16f8fab))
*   add tags ([9193a3bb](https://github.com/Egomination/mangapplizer-backend/commit/9193a3bbd54be9c6206afbd467bed2d256375a70))
*   add genres ([d55309ca](https://github.com/Egomination/mangapplizer-backend/commit/d55309caffd8072145e39f49c4229f46925f8776))
*   add env logger ([c3faed07](https://github.com/Egomination/mangapplizer-backend/commit/c3faed07e5cb674b0cdce0d8e50d1d1ea4db9ae4))
*   add slug service for finding manga using id ([0f06feb3](https://github.com/Egomination/mangapplizer-backend/commit/0f06feb30a42cb95e9a0996548c027f060b8134b))
*   add response struct for json response ([847584b7](https://github.com/Egomination/mangapplizer-backend/commit/847584b766366b8687e6641146e9e2be5a514b03))
*   add relations into the response ([5e21c87d](https://github.com/Egomination/mangapplizer-backend/commit/5e21c87d32d9fee7d294ec49626666b61ce51ce0))
*   remove psql folder and create new one on each run ([7f5d90d6](https://github.com/Egomination/mangapplizer-backend/commit/7f5d90d6a4e59b92a068f1c81b6b551b5c4d4db7))
*   add find(full) function ([37efd8aa](https://github.com/Egomination/mangapplizer-backend/commit/37efd8aaaa92f032845a8e8e1d416722f72b086a))
*   add find route for finding manga from id ([0ec69430](https://github.com/Egomination/mangapplizer-backend/commit/0ec69430ba5df1fd779ea6e9d11decf9500dc384))
*   improve manga insertion to the database ([d62f7ed5](https://github.com/Egomination/mangapplizer-backend/commit/d62f7ed55181f391124fc434136a5710f1da6f5c))
*   add create implementation to media struct ([f40d0d4c](https://github.com/Egomination/mangapplizer-backend/commit/f40d0d4caffaa1d9bd9ec6416f6c039c79370780))
*   implement create function for relation struct ([fd2d4143](https://github.com/Egomination/mangapplizer-backend/commit/fd2d41436d3d6f53e5313547369766d332719148))
*   add create function for the new series struct ([792bd0b7](https://github.com/Egomination/mangapplizer-backend/commit/792bd0b7a0eedda35bb0c4b10029cf999fec808f))
*   enable insertion on bridge table ([ed650b19](https://github.com/Egomination/mangapplizer-backend/commit/ed650b19084243bd839831fde8042701584c3ae9))
*   add staff insertion method ([3eb4b5dc](https://github.com/Egomination/mangapplizer-backend/commit/3eb4b5dc58700e3e555ee78256a8310dc159a5d6))
*   add basic http server and handlers ([6febce49](https://github.com/Egomination/mangapplizer-backend/commit/6febce49924a81c49ffcd7454c7eb68fb2e92712))
*   add actix and actix-web ([2d9b2c62](https://github.com/Egomination/mangapplizer-backend/commit/2d9b2c626e398a6ff0aee571bdebbe128c89e82f))
*   add connection pool r2d2 ([4dd39791](https://github.com/Egomination/mangapplizer-backend/commit/4dd3979166da9c036161e77ce3180315474e8a4d))
*   add primitive search function from db ([05f7cc6e](https://github.com/Egomination/mangapplizer-backend/commit/05f7cc6e640ae2a186f848a6773b673cfa063440))
*   add basic get logic from db ([bc28c463](https://github.com/Egomination/mangapplizer-backend/commit/bc28c4632b36d9aa23f78f0fa847e6c822fb5c9f))
*   add migration folders and db function implementations ([0ec1cf7a](https://github.com/Egomination/mangapplizer-backend/commit/0ec1cf7afcf15207026c339e4adb24e3b4956fe6))
*   add postgresql docker runner script ([4aeaed29](https://github.com/Egomination/mangapplizer-backend/commit/4aeaed29e01b82e9d568dc6abcd1c49da4966047))
*   add example files for http parsing ([13cc2d50](https://github.com/Egomination/mangapplizer-backend/commit/13cc2d5053e70c861ddd4e818daa48aaab5aa3f3))
*   flush go ([ebdf242c](https://github.com/Egomination/mangapplizer-backend/commit/ebdf242ccce9b796f6e4b747d6f24ce676b752f2))
*   remove old packages and files ([94f79567](https://github.com/Egomination/mangapplizer-backend/commit/94f79567cef5f4029126b581781b551565f852f3))
*   travis ([6080cea0](https://github.com/Egomination/mangapplizer-backend/commit/6080cea078cc24522831be809f6306fc38e60bb3))
*   Anilist data fetch and Chores ([f21ea559](https://github.com/Egomination/mangapplizer-backend/commit/f21ea55958e4c5f3d63bca99352be52b6f56b6e8))
*   travis ([7a5d939a](https://github.com/Egomination/mangapplizer-backend/commit/7a5d939a8b33f2a047bb370ea7d9a3339fda97b0))
*   Anilist data fetch and Chores ([2a86cf5f](https://github.com/Egomination/mangapplizer-backend/commit/2a86cf5fd6fe59f6198a58055c380ee606506908))
* **core:**
  *  switch to swagger ([da4acbc3](https://github.com/Egomination/mangapplizer-backend/commit/da4acbc3be24825f809861ec9b2ad0854c3f5c4c))
  *  add main logic for route handlers ([374853a0](https://github.com/Egomination/mangapplizer-backend/commit/374853a0be801302ffcd6933c8a783a77ec091ca))
  *  add main logic for routing ([7ee4dbbc](https://github.com/Egomination/mangapplizer-backend/commit/7ee4dbbc5ca5319aa3ca9cefd9e9060dc9efdeba))
* **database:**
  *  add ability to insert into database ([cd7c5239](https://github.com/Egomination/mangapplizer-backend/commit/cd7c5239f20d6281ec9fb37cd3d428d99931ff82))
  *  add sql schema ([fa6ed372](https://github.com/Egomination/mangapplizer-backend/commit/fa6ed37289bcf37a0b969cb701874f3780dddb0d))
  *  add mock tables for testing gorm's table creation ([f517fe57](https://github.com/Egomination/mangapplizer-backend/commit/f517fe57f2d8ceb67348467d0cb923d83330f296))
  *  add gorm orm with postgresql flavour ([44f86b9a](https://github.com/Egomination/mangapplizer-backend/commit/44f86b9aa654b884826cc1e9554baa27b4b53e8d))
* **docker:**  add script for running database containers ([65fda37c](https://github.com/Egomination/mangapplizer-backend/commit/65fda37cf55b57d500a3f46b4565f43e4c94001a))
* **parser:**
  *  add mangarock search ([2fe53125](https://github.com/Egomination/mangapplizer-backend/commit/2fe5312577d2328574a5732cfe92ff1e6b21ec26))
  *  add mangarock chapter saver & mri to png converter ([19dae5d3](https://github.com/Egomination/mangapplizer-backend/commit/19dae5d3683798f72a24008fbb7d823de054a8ec))
  *  add mangarock as a chapter source ([185b8cb7](https://github.com/Egomination/mangapplizer-backend/commit/185b8cb7675248d87f95ec35115187bce1f1ccd6))
  *  add mangarock ([a3da8948](https://github.com/Egomination/mangapplizer-backend/commit/a3da8948fc1c95839f4be94c7d7f443322efec2c))
  *  add mri decoder for mangarock ([213fddb8](https://github.com/Egomination/mangapplizer-backend/commit/213fddb84eda127990cf6049eebbc207ab9a0e6f))
  *  add mangarock ([42ea4965](https://github.com/Egomination/mangapplizer-backend/commit/42ea496512706a2adc4c67f36f8b2fc27a8b5316))



