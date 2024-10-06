# hoyolab_login_manager
hoyolab_login_manager は "原神", "崩壊:スターレイル" をプレイする人のためのDiscord botです.
![image](https://github.com/AttODao/hoyolab_login_manager/assets/127784728/ee64adad-fe9c-49c3-b89b-9ac2dc87ed95)
![image](https://github.com/AttODao/hoyolab_login_manager/assets/127784728/1e4d39a6-e901-40f1-b4a4-6f7e699d95a5)

機能:
- ログインボーナスの自動受取
- 樹脂・開拓力等の通知

# コマンド一覧
| コマンド    | 引数                                  | 説明                                                                                                                  |
| ----------- | ------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| register    | game: ゲームの選択</br>id: ユーザーID | DiscordのアカウントとゲームのユーザーIDの紐づけを行います.                                                            |
| cookie      | ltoken</br>ltuid                      | HoYoLABのCookieのltoken, ltuidの部分を登録します. これを登録しないとログインボーナスの自動受取や通知を利用できません. |
| daily set   | setting: 有効・無効の設定             | ログインボーナスの自動受取の有効・無効を設定します.                                                                   |
| daily info  | (game: ゲームの選択)                  | ログインボーナスの自動受取の設定状況・ログインボーナスの受け取り状況を確認します.                                     |
| daily claim | (game: ゲームの選択)                  | ログインボーナスを受け取ります.                                                                                       |
| notify set  | setting: 有効・無効の設定             | 通知の有効・無効を設定します.                                                                                         |
| notify info | -                                     | 通知の設定状況を確認します.                                                                                           |

# botの設定 (config.ymlファイル)
| 設定                         | 説明                                            |
| ---------------------------- | ----------------------------------------------- |
| discord_token                | Discordのbot token                              |
| database_url                 | postgresqlのurl                                 |
| lang                         | HoYoLABの言語                                   |
| scheduler_interval_mins      | 処理を行う間隔(分)                              |
| claim_daily_time             | ログインボーナスの自動受け取りを行う時刻        |
| gensin_resin_notify_mins     | 原神の樹脂の通知を何分前に行うか                |
| gensin_home_coin_notify_mins | 原神の洞天宝銭の通知を何分前に行うか            |
| starrail_stamina_notify_mins | 崩壊:スターレイルの開拓力の通知を何分前に行うか |

設定例:
- config.yml
```yml
discord_token: bot_token
database_url: postgres://postgres:Password@localhost/database
lang: ja-jp
scheduler_interval_mins: 5
claim_daily_time: 08:00:00
genshin_resin_notify_mins: [0,30,60,180]
genshin_home_coin_notify_mins: [0,60,180,360]
starrail_stamina_notify_mins: [0,30,60,180]
```
- docker-compose.yml
```yml
version: '3'
services:
  db:
    image: postgres
    restart: always
    environment:
      POSTGRES_DB: database
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: Password
    ports:
      - 5432:5432
    volumes:
      - db:/var/lib/postgresql/data
  app:
    image: hoyolab_login_manager
    build: .
    restart: always
    volumes:
      - ./config.yml:/config.yml
    depends_on:
      - db

volumes:
  db:
```
