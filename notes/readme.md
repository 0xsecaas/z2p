# The Problem
Build an **email newsletter** service that supports what you need to get off the ground if you are willing to add an email subscription page to your blog.

## Requirements 
### User Stories:

```> [!NOTE]
> As a ..., I want to ..., So that ...

A user story helps us to capture:
- Who we are building for (as a)
- The actions they want to perform (want to)
- Their motives (so that).


#### U1
As a blog visitor, 
I want to subscrube to the newsletter, 
So that I can receive email updates when new content is published on the blog.

#### U2
As the blog author, 
I want to send an email to all my subscribers,
So that I can notify them when new content is published.

#### U3
As a subscriber,
I want to be able to unsubscribe from the newsletter,
So that I can stop receiving email updates from the blog.

### Out of the Scope
- Manage multiple newsletters
- Segment subscribers in multiple audiences
- Track opening and click rates.

### Implicit Requirements
Looking closer to the requiremnets, we realize the following questions:
1. how do we ensure that the caller is indeed the blog author?
2. do we need to introduce an authentication mechanism?
3. do we support HTML in emails or do we stick to plain text? what about emojis?

We should build enough functionality to satisfy, to an extent the requirements of all of our stories in our first release -- the MVP. We will then go back and improve by adding:
- fault-tolerance
- retries for email delivery
- confirmation email for new subscribers, etc.

We will work in iterations: each iteration takes a fixed amount of time and gives us a slightly better version of the product, improving the experience of our users.

> We iterate on features, not engineering quality: the code produced at each iteration is ready to ship to production -- tested and properly documented.
---

## Plan
We expect our blog visitors to input their email address in a form embedded on a web page.
The form will trigger an API call to a backend server that will actually process the information, store it and send back a response.`subscriptions POST` endpoint.

