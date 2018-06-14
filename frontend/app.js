function item(data) {
    if (data == null) { return ""; }
    var result =  "<ul><li>" + data["sender_name"] + " :: ";
    switch (data["post_type"]) {
      case 0: 
        result += data["content"];
        break;
      case 1:
        result += "<img src=\"https://server.shaneqi.com/public/" + data["content"] + "\"></img>";
        break;
    }
    $.each( data["children"], function( key, val) {
        result += item(val);
    });
    
    result += "</li></ul>";
    return result;
}
$.getJSON( "https://server.shaneqi.com/telepost/", function( data ) {
  var items = [];
  $.each( data, function( key, val ) {
    items.push(item(val));
  });
 
  $( "<div/>", {
    html: items.join( "" )
  }).appendTo( "body" );
});
