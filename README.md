# thinkcoin 
-----
Earn money by solving puzzles/chess problems!

# TODO
- implement bitcoin protocol
- create proof of work system

# Steps from bitcoin whitepaper
- New transactions are broadcast to all nodes.
- Each node collects new transactions into a block.  
- Each node works on finding a difficult proof-of-work for its block.
- When a node finds a proof-of-work, it broadcasts the block to all nodes.
- Nodes accept the block only if all transactions in it are valid and not already spent.
- Nodes express their acceptance of the block by working on creating the next block in the 
- chain, using the hash of the accepted block as the previous hash.

# Resources
- [https://www.jmeiners.com/tiny-blockchain/](https://www.jmeiners.com/tiny-blockchain/)
- [https://en.bitcoin.it/wiki/Protocol_documentation](https://en.bitcoin.it/wiki/Protocol_documentation)
- [https://learnmeabitcoin.com/](https://learnmeabitcoin.com/)
- [https://developer.bitcoin.org/devguide/block_chain.html](https://developer.bitcoin.org/devguide/block_chain.html)
