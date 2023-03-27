javascript:(function(){
let text;
  let person = prompt("Aramak istediğin tagı yaz:", "derin tech");
  if (person == null || person == "") {
    text = "User cancelled the prompt.";
  } else {
    location.href='https://erank.com/keyword-tool?keywords='+ person +'&country=USA';
  }})();


  javascript:(function(){ 
    var current_url = location.host + location.pathname + location.search; 
    redirect_link = encodeURIComponent(current_url); 
    location.href='http://erank.com/analyzer?link=%27+redirect_link;})();