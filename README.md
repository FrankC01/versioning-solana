## versioning-solana

This repo demonstrates ONE rudimentary way to upgrade/migrate account data changes with solana program changes.

## What is data versioning?
Fundamentally to version data means to create a unique reference for a collection of data. This reference can take the form of a query, an **ID**, or also commonly a datetime identifier.

### Simple scenario:
1. You create a program that stores a `u64` value in your program's accounts data, assume you initialized the account and set `somevalue` to `50u64`

<img src="images/versioning-solana-v0.png" alt="alt text" width="860" height="440">

2. Later on you decide that you want to also have some `String` value to reside in the account data that the rest of your program requires to be present



What do you do? There are a few options:

1. If the initial allocation of your program account has room to spare, and you had the foresite to include a 'data version' indicator in the account data: This repo will demonstrate an approach

2. If the initial allocation was sized specifically to the `u64`:

    * If you are running with a Solana version that incorporates the 'account re-allocation feature' then leverage that
    * Otherwise, leveraging PDAs may help but someone else will have to demonstrate that.

### On with the show
<img src="images/best_practice.png" alt="alt text" width="60" height="40"> In your initial planning, make sure to include a 'data version' field in your data. It can be a simple incrementing ordinal (e.g. u8) or something more sophisticated, up to you.
