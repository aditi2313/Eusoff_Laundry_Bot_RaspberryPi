var token = "1676131447:AAG6wp9M41tZtCSuy1pR_BU2Odn-FyH0mgA";
var telegramUrl = "https://api.telegram.org/bot" + token;
var webAppUrl = "https://script.google.com/macros/s/AKfycbwoWJBItnwHwz-OwfO80L84edZgqugLql085ModARRnnDVlkjI/exec";

function setWebhook(){
  var url = telegramUrl + "/setWebhook?url=" + webAppUrl;
  var response = UrlFetchApp.fetch(url);
}  

function sendMessage(id, text, keyBoard){
  var data = {
    method: "post",
    payload: {
      method: "sendMessage",
      chat_id: String(id),
      text: text,
      parse_mode: "HTML",
      reply_markup: JSON.stringify(keyBoard)
    }
  };
  UrlFetchApp.fetch('https://api.telegram.org/bot' + token + '/', data);
}

function doPost(e){
  var contents = JSON.parse(e.postData.contents);
  var ssId = "1tzrl93Nn-Ce8iyI9f13PUMYd1PtJ9g064yxk3Isbtdc";
  var sheet = SpreadsheetApp.openById(ssId).getSheetByName("Laundry Machines");
  var number_keyBoard = {
        "inline_keyboard": [
          [{
            "text": "Machine 1",
            "callback_data": 1
          }],
          [{
            "text": "Machine 2",
            "callback_data": 2
          }],
          [{
            "text": "Machine 3",
            "callback_data": 3
          }],
          [{
            "text": "Machine 4",
            "callback_data": 4
          }],
          [{
            "text": "Machine 5",
            "callback_data": 5
          }],
          [{
            "text": "Machine 6",
            "callback_data": 6
          }],
        ]
  }
  
  var sample_keyBoard = {
     "keyboard": [
       [ 
         "Check machine status :D"
       ]/*,
       [
         "Notify :)"
       ]*/
     ],
     resize_keyboard:true,
     one_time_keyboard:false
   }


  if (contents.callback_query){
    var id = contents.callback_query.from.id;
    var username = contents.callback_query.from.username;
    var data = contents.callback_query.data;
       for (i = 1; i < 7; i++){
          if (i == data){
              var using = sheet.getDataRange().getCell(9, i + 1).getValue();
              var status = sheet.getDataRange().getCell(3, i + 1).getValue();
              var statusarr = status.split(',');
              var counter = 0;
              for (j = 0; j < statusarr.length; j++){
                if (statusarr[j] == "ON"){
                  counter = counter + 1;
                }
              }
              if (counter > 0 && using != ""){
                  var return_text = "Sorry... Machine " + i + " is currently in use... Please try again later D:";
              } else {
                  sheet.getRange(9, i + 1).setValue("@" + username);
                  sheet.getRange(10, i + 1).setValue(id);
                  var return_text = "Machine " + i + " has been blocked under your name... You will be notified when your laundry is done!";
              }
              sendMessage(id, return_text);
          } 
       }
  } else if (contents.message){
    var id = contents.message.from.id;
    var text = contents.message.text;
    if (text == "Check machine status :D"){
      var checked = sheet.getDataRange().getCell(4, 2).getValue();
      let [h, m, s] = checked.toLocaleTimeString("en-GB").split(/:| /);
      var return_text = "";
      for (i = 1; i < 7; i++){
        var statusline = sheet.getDataRange().getCell(3, i + 1).getValue();
        var statusarr = statusline.split(',');
        var counter = 0;
        for (j = 0; j < statusarr.length; j++){
          if (statusarr[j] == "ON"){
             counter = counter + 1;
          }
        }
        if (counter == 0){
          return_text = return_text + "\nMachine " + i + ": Available :D\n";
        } else {
          var runtime = sheet.getDataRange().getCell(7, i + 1).getValue();
          let [hour, minute, second] = runtime.toLocaleTimeString("en-US").split(/:| /);
          return_text = return_text + "\nMachine " + i + ": In use. Running for " + hour + " hour(s) and " + minute + " min(s) O.o\n";
         }
      }
      return_text = return_text + "\n~ Last checked at " + h + ":" + m + " hrs.\n";
      sendMessage(id, return_text);/*
    } else if (text == "Notify :)"){
      return sendMessage(id, "Select the machine to reserve ._.", number_keyBoard);*/
    } else {
      return sendMessage(id, "Hi there! Welcome to the Eusoff Laundry Bot... What would you like to do today? :P", sample_keyBoard);
    }
  }
}      
