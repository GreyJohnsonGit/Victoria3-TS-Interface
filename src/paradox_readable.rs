pub trait ParadoxReadable {
  fn pdx_token_callback(&self, token: String) -> String;
}