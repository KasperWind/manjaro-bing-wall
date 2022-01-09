# manjaro-bing-wall
Manjaro bing wallpaper daemon

## Getting image

Bing api to get picture of the day: http://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US
Bing pictures downloaded from the url returned from images[].url
appended to www.bing.com/
``` 
 {
  "images": [
    {
      "startdate": "20141214",
      "fullstartdate": "201412141830",
      "enddate": "20141215",
      "url": "\/az\/hprichbg\/rb\/BlackButte_EN-IN7038391888_1920x1080.jpg",
      "urlbase": "\/az\/hprichbg\/rb\/BlackButte_EN-IN7038391888",
      "copyright": "Black Butte, seen from the Mount Jefferson Wilderness, Oregon, USA (\u00a9 Marc Adamus\/Aurora Photos)",
      "copyrightlink": "http:\/\/www.bing.com\/search?q=Black+Butte&qs=n&form=hpcapt&mkt=en-in&pq=black+butte&sc=8-11&sp=-1&sk=&cvid=228ac7f125f94bbaafd4a4abd4f9a32d",
      "wp": true,
      "hsh": "94156ae1e2e1be49f9b739d2b7bff65c",
      "drk": 1,
      "top": 1,
      "bot": 1,
      "hs": [

      ],
      "msg": [
        {
          "title": "How does it feel\u2026",
          "link": "http:\/\/www.bing.com\/videos\/search?q=Climbing+Black+Butte&FORM=pgbar1&mkt=en-in#view=detail&mid=58BDB2F2B9FCB85D597558BDB2F2B9FCB85D5975",
          "text": "To climb 1961.7 m?"
        },
        {
          "title": "On top of the world",
          "link": "http:\/\/www.bing.com\/images\/search?q=Pictures+From+the+Top+of+Mount+Everest&FORM=pgbar2&mkt=en-in",
          "text": "It's mountaineer's dream view"
        }
      ]
    }
  ],
  "tooltips": {
    "loading": "Loading...",
    "previous": "Previous",
    "next": "Next",
    "walle": "This image is not available to download as wallpaper.",
    "walls": "Download this image. Use of this image is restricted to wallpaper only."
  }
}
```
```
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    let uri2 = "http://www.bing.com//th?id=OHR.WinterBison_EN-US1438810655_1920x1080.jpg&rf=LaDigue_1920x1080.jpg&pid=hp".parse()?;
    let mut resp2 = client.get(uri2).await?;

    println!("Response image: {}", resp2.status());


    if false {
        while let Some(chunk) = resp2.body_mut().data().await {
            stdout().write_all(&chunk?).await?;
        }
    }

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    if let Some(data) = resp.body().data(). {

        data
    }
```

## Change wallpaper 
xfconf-query -c xfce4-desktop -p /backdrop/screen0/monitor0/workspace0/last-image

set: /backdrop/screen0/monitorHDMI-0/workspace1/last-image
set: /backdrop/screen0/monitorHDMI-0/workspace1/last-image


