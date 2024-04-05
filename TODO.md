# TODO
Everything that's left to do in the project

- [ ] Set up proper testing on the project
    - [ ] File Tracking
    - [ ] The server itself???
- [ ] Add better documentation comments (document the modules, for example)

## User Management
- [ ] Allow the server to have a password so not anyone can join
- [ ] Implement some type of random username when two users have the same username
- [ ] Allow users to set a color to their cursor

## File Management
- [ ] Implement some sort of .gitignore for files that don't want to be shared in the network

## File editing
- [ ] Send the content of the files to the client when a user joins to actually edit them
    - [ ] Send them compressed and let the client decompress them for more network performance
    - [ ] Maybe send first the files with the most users editing them so they can be edited faster
- [ ] Allow the users to send changes to files, containing only the parts changed and not the entire new file produced.
- [ ] Store the changes made by each user in a undo tree
    - [ ] Allow to remove specific (or all) changes made by a user
    - [ ] Allow to return to an earlier state of the file
    - [ ] Option for removing all changes made

## Options for the server CLI
- [ ] Initial args
    - [ ] path to shared directory/file (if none provided, just use the current working directory)
    - [ ] (optional) password
    - [ ] (optional) .fileignore => Just a file in the directory that's gonna be 
            shared with the files/directories that don't want to be shared accross the clients
- [ ] Commands while server is executing
    - [ ] ban => Ban user (By socket id or by username, haven't decided yet)
    - [ ] passwd => Change password
    - **UNDO STUFF GOES HERE**

## Docs
- [ ] Create a small documentation page (with docs.rs in some sort of way, or starlight, who knows) containing the messages the server accepts and returns
