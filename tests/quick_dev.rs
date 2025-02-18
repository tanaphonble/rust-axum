use anyhow::Result;

#[tokio::test]
async fn default_hello_world() -> Result<()> {
    
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
 
    Ok(())
}

#[tokio::test]
async fn hello_with_name() -> Result<()> {
    
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=ble").await?.print().await?;
 
    Ok(())
}