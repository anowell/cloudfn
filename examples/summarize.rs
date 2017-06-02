extern crate cloudfn;

use cloudfn::nlp::*;

fn main() {
    let text = r#"
    Fans, for the past two weeks you have been reading about a bad break I got. Yet today I consider myself the luckiest man on the face of the earth. I have been in ballparks for seventeen years and have never received anything but kindness and encouragement from you fans.

    Look at these grand men. Which of you wouldn’t consider it the highlight of his career to associate with them for even one day?

    Sure, I’m lucky. Who wouldn’t consider it an honor to have known Jacob Ruppert – also the builder of baseball’s greatest empire, Ed Barrow – to have spent the next nine years with that wonderful little fellow Miller Huggins – then to have spent the next nine years with that outstanding leader, that smart student of psychology – the best manager in baseball today, Joe McCarthy!

    Sure, I’m lucky. When the New York Giants, a team you would give your right arm to beat, and vice versa, sends you a gift, that’s something! When everybody down to the groundskeepers and those boys in white coats remember you with trophies, that’s something.

    When you have a wonderful mother-in-law who takes sides with you in squabbles against her own daughter, that’s something. When you have a father and mother who work all their lives so that you can have an education and build your body, it’s a blessing! When you have a wife who has been a tower of strength and shown more courage than you dreamed existed, that’s the finest I know.

    So I close in saying that I might have had a tough break – but I have an awful lot to live for!
    "#;

    let resp = classifier4j::summarize(text).unwrap();
    println!("Classifier4J: {}", resp);

    let resp = summarai::summarize(text).unwrap();
    println!("Summarai: {}", resp.summary);
    println!("Summarai keywords: {:?}", resp.keywords);

}
