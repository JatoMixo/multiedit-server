# Server Documentation
## Messages sent by the clients
- `join` => Request sent after establishing the websocket connection, it sends the configuration and data
            of the client connected to the server, which includes:
    - `username` => The username chosen by the client

    **Possible responses**:
    - `user-creation-error` => UserCreationError
    - `file-content-tree` => HashMap<Path, String>
    - `file-content-error` => String for now
- `apply-change` => Apply a change to a file, it takes a **FileChange**:
    - `file` => The file to which the change was applied
    - `start_index` => The starting index of the change
    - `end_index` => The index of the end of the change, the character in this index is not included in the change
    - `content` => The content that replaces the characters between **start_index** and **end_index**

    **Possible responses**:
    - `file-error` => FileTrackingError
### Still need to be implemented
- `move-cursor` => Move the cursor to a different position of the file, it takes:
    - `column` => The new column of the cursor
    - `row` => The new row of the cursor

## Messages sent by the server
- `change-applied` => Other user applied a change to a file, it returns:
    - `file` => The file to which the change was applied
    - `start_index` => The starting index of the change
    - `end_index` => The index of the end of the change, the character in this index is not included in the change
    - `content` => The content that replaces the characters between **start_index** and **end_index**
- `client-connected` => Indicates another client connected, to start rendering his mouse.
    - `client_id` => The ID of the connected client
    - `username` => The username of the connected client
- `client-disconected` => Indicates another client disconnected, to remove their cursor from the screen.
    - `client_id` => The ID of the disconnected client
### Still need to be implemented
- `cursor-moved` => Indicates a user moved their cursor, it returns:
    - `client_id` => The ID of the client that moved the cursor
    - `column` => The new column of the cursor
    - `row` => The new row of the cursor

