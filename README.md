# Hello, help please?

In my efforts to learn rust (and tell a heck of a cool story about the inner SRE culture of GOOGLE) I've been trying to write a program to bridge Google Cloud PubSub to MQTT. While writing that program, I've had to learn a lot of Rust. This is the first time
I've hit a problem I can't read my docs or experiment through. I promise that if someone helps me understand, I'll put up a more thorough and helpful explanation somehwere that is discoverable and adequately SEO'd so that other people will struggle less with tis.

## The Problem

The problem is simple: This program doesn't compile. I want to pass a tokio `mpsc` channel into the `Fn` that this google cloud pubsub library uses to receive and process messages. The reasoning behind this request should be straightforward, and it seems like
a very reasonable thing to want to do.

I simply don't understand how it's even possible to do it at all with this API, except maybe with the [lazy_static]() crate which I have't tried yet, and that itself would be extremely dicey to use in this case.

What's the Rust-ish way here?

## My Workaround

Currently I do have a workaround that entirely avoids this call. There's [an alternative way to get a stream of values out](https://docs.rs/google-cloud-pubsub/0.11.0/google_cloud_pubsub/subscription/struct.Subscription.html#method.subscribe). However, I feel 
like I'm brushing up against something Fundamental and Important here, and I'd rather not run away from every difficult edge of Rust if I'm really making an effort to learn it.

But that does mean: this isn't urgent. I'm not slowly withering away dying with the need to know. This code isn't mission critical or even that important. I'd just like to solve my problem and then write up a piece helping other people understand the issue.


