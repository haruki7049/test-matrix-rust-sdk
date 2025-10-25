// OidcRequest は v0.7.0 の sso-login フローでは使用しない
// SyncSettings はこの例では未使用
// tiny_http 関連 (Arc, Server, Response) は不要
use matrix_sdk::{
    Client,
    ruma::UserId, // UserId 型注釈のために必要
    authentication::matrix::MatrixAuth,
};
use url::Url; // Url 型注釈のために必要

// OIDCのリダイレクトURI。
const REDIRECT_URI: &str = "http://localhost:8080/auth/callback";

#[tokio::main]
// main の戻り値の型を、複数のエラー型 (ClientBuildError, OidcError, ParseError) を
// 扱えるように Box<dyn std::error::Error> に変更
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // 1. ホームサーバーのURLを設定
    let homeserver_url: &str = "https://matrix.org";

    // 2. Client の構築
    // .build().await? の結果、型は `Client` となる
    let client: Client = Client::builder()
        .homeserver_url(homeserver_url)
        .build()
        .await?; // ClientBuildError を `?` で伝播

    // 3. OIDCハンドラの取得
    // .oidc() メソッドは `Oidc` 型（所有値）を返す
    let matrix_auth: MatrixAuth = client.matrix_auth();

    // (修正) ビルダー自体を .await して Url を取得
    let login_url: Url = Url::parse(&matrix_auth.get_sso_login_url(REDIRECT_URI, None).await?)?;

    println!("----------------------------------------------------------------");
    println!(" harukiさん、以下のURLにブラウザでアクセスしてログインしてください:");
    println!("{}", login_url);
    println!("----------------------------------------------------------------");

    // 5. コールバックの待機とログイン完了
    // `sso-login` feature により、finish_login が内部で
    // ローカルサーバーを起動し、コールバックを待機する。
    println!("ローカルサーバー (http://127.0.0.1:8080) でコールバックを待機中..."); // (修正) 120.0.0.1 -> 127.0.0.1

    // .finish_login() は引数を取らず、内部でコールバック受信と検証、
    // トークン取得まで行う。 Result<(), OidcError> を返す Future。
    oidc.finish_login().await?;

    println!("コールバック受信成功。ログインを完了しました。");

    // 6. ログイン確認
    if client.logged_in() {
        // .user_id() は Future ではなく `Option<&UserId>` を返す
        // .await は不要
        let user_id: &UserId = client.user_id().unwrap();
        println!("\nログイン成功！");
        println!("ユーザーID: {}", user_id);

        // (参考) 必要に応じて初回同期などを実行
        // use matrix_sdk::config::SyncSettings;
        // client.sync_once(SyncSettings::default()).await?;
        // println!("同期完了。");
    } else {
        println!("\nログインに失敗しました。");
    }

    Ok(())
}

// `wait_for_callback` 関数は sso-login feature が
// 内部で処理してくれるため、丸ごと不要。


