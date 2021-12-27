
mod theme;

use std::error::Error;
use hnapi::{get_articles, Articles};


fn render_articles(articles: &Articles){
    let theme = theme::default();
    theme.print_text("# Top Stories\n\n");
    for i in &articles.hits{
    
        if  i.url.is_some(){

            theme.print_text(&format!("`{}`", i.title));
            theme.print_text(&format!("> *{}*", i.url.as_ref().unwrap()));
            theme.print_text("---");

           
        } else {

            theme.print_text(&format!("`{}`", i.title));
            theme.print_text("---");


        }
       
        
    }

}


fn main() -> Result<(), Box<dyn Error>> {
    
    let url: &str = "http://hn.algolia.com/api/v1/search?tags=front_page";
    let articles = get_articles(url)?;


    render_articles(&articles);

    Ok(())

   
}
