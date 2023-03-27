javascript:(function(){
let text;
  let person = prompt("Aramak istediğin tagı yaz:", "derin tech");
  if (person == null || person == "") {
    text = "User cancelled the prompt.";
  } else {
    location.href='https://erank.com/keyword-tool?keywords='+ person +'&country=USA';
  }})();