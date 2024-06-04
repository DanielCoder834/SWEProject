**Statement of Work - Team 8**

Before starting to code, each team should write a SOW that describes what they expect to implement in terms of MVP (minimal viable product), desirable features and additional features. You will be graded based on meeting that SOW. Use best-practices to document the SOW.

Requirements (MVP):

- Client:
  - Sheets that can compute values of cell based on formulas
  - Mediate interactions with UI and the server
- GUI
  - Ability to display and edit sheets
- Documentation on how to use it
- Server:

○ Provide endpoint functionality

Additional Features:

- Theme Picker Picker
- Independent Save History
- Copying Sheets
- Private Sheets

Endpoints:

- register() causes the server to create a publisher with the client name. No value is returned.
- getPublishers() returns a list of argument objects with the publisher field set to all registered publishers.
- getSheets(arg) takes an argument object with field publisher set to the name of a publisher and returns a list of argument objects with the publisher and sheet fields set to all sheet names for the given publisher.
- createSheet(arg) takes an argument object with fields publisher and sheet set to the name of the client and the name of a sheet to create. No value is returned.
- deleteSheet(arg) takes an argument object with fields publisher and sheet set to the name of the client and the name of a sheet to delete. No value is returned.
- getUpdatesForSubscription(arg) takes an argument object with fields publisher, sheet and id set to the name of a publisher, a sheet, and an id. It returns an argument object with the payload set to all updates that occurred after id, and the id field set to the last id for those updates. The sheet is owned by a publisher different from the client. An empty payload is returned if no updates occurred after the given id.
- getUpdatesForPublished(arg) takes an argument object with fields publisher, sheet and id set to the name of a publisher, a sheet, and an id. It returns an argument object with the payload set to all the requests for updates that occurred after id, and the id field set

  to the last id for those requests for updates. The sheet is owned by the client. An empty payload is returned if no updates occurred after the given id.

- updatePublished(arg) takes an argument object with fields publisher, sheet and payload set to the name of a publisher, a sheet, and updates for that sheet. No value is returned. The sheet is owned by the client.
- updateSubscription(arg) takes an argument object with fields publisher, sheet and payload set to the name of a publisher, a sheet, and requests for updates for that sheet. No value is returned. The sheet is owned by a publisher different from the client.

**User Stories**

1. As a supplier, I require my subscribers to register a unique name and to follow a set of instructions when creating their passwords, so that the application can avoid the same username errors and users have their data protected. (MVP - High)

**Acceptance Criteria** (conditions of satisfaction)

- **No Same Names:** Users must choose a unique username when registering.
- **Data Protection:** Users must follow a set of guidelines that enforce strong passwords when registering.
- **Error Handling**: If a user chooses a username that has already been taken by another user, an error message will be displayed.
- **Feedback:** On a successful registration, a visual indication will be displayed.
- **Login**: To login, users must enter their unique username and unique password to gain access to their spreadsheets.
- ---
2. As a subscriber, I want to register a unique name and password through a GUI so that my data can be protected. When I log-in I want to be able to use my registered username and password to access my data. (MVP - High)

**Acceptance Criteria** (conditions of satisfaction)

- **No Same Names:** Don’t allow users to register under a name that has already been used.
- **Persistency:** Logging in will allow users to see their previously used and saved sheets.
- **Error Handling:** If a user selects a username that is already being used by another user, an error message will be displayed.
- **GUI:** Visual representation that labels where to enter a username and password for registering or logging in.
- **Feedback:** On successful log-in, a visual indication will be displayed.
- ---
3. As a subscriber, I want to see the list of registered users, so that I can view their sheets and contribute to them. (MVP - High)

**Acceptance Criteria** (conditions of satisfaction)

- **Correctness:** Display the username of each registered user.
- **Speed:** Shouldn’t take too long to return the list
- **GUI:** Visual representation and area to see the list of users (drop-down?)
- ---
4. As a subscriber, I want to see the list of sheets each registered user is the publisher of, so that I can contribute to their sheets by subscribing to them. (MVP - High)

**Acceptance Criteria** (conditions of satisfaction)

- **Correctness:** Display the list of spreadsheets the selected publisher is the owner of.
- **Speed:** Shouldn’t take too long to return the list
- **GUI:** Visual representation and area to see the list of spreadsheets the publisher owns (drop-down?)
- **Privacy:** Don’t show private sheets of publishers
- ---
5. As a subscriber, I want to create sheets with specific names so that I can maintain and organize data. (MVP - High)

**Acceptance Criteria** (conditions of satisfaction)

- **Naming:** The name of the spreadsheet is correct as provided by the user.
- **Publisher Status:** The publisher is set to the user who created it.
- **Privacy:** Can set privacy settings (public / private)
- **GUI:** Visual representation and area for users to create their own sheets and name them.
- ---
6. As a supplier, I want to store the sheets that the user creates, so that they have persistent data storage. (Desirable Features - High)

**Acceptance Criteria** (conditions of satisfaction)

- **Updated List of Sheets**: Newly created sheet is added to the user’s list of sheets they have access to.
- **Privacy:** Store the privacy level for this sheet and don’t allow other users to see it if it’s set to private.
- **Persistency:** After being created, if the user logs out or exits the spreadsheet, all of the data is maintained and consistent.
- ---
7. As a subscriber, I want to delete spreadsheets so that I can organize my data well and get rid of spreadsheets I don’t use anymore. (MVP - High)

**Acceptance Criteria** (conditions of satisfaction)

- **GUI:** Visual representation, visual feedback, and area provided when deleting sheets. Also provide a “are you sure?”
- **Disappearance:** Users can no longer view or find deleted sheets.
- ---
8. As a supplier, I want to allow users to delete spreadsheets, so that they can organize and maintain their data in the way they see fit. (MVP - High)

**Acceptance Criteria** (conditions of satisfaction)

- **Persistency:** Deleted spreadsheets are still deleted after relogging or refreshing
- **Updated List of Sheets**: Deleted sheets are removed from the user’s list of sheets they have access to.
- **Data Removal:** All data in the deleted spreadsheet is discarded.
- ---
9. As a subscriber, I want to retrieve a list of updates from other subscribers on a specific spreadsheet, so that I can view the work of others and ensure that there are less data conflicts. (MVP - Medium)

**Acceptance Criteria** (conditions of satisfaction)

- **Correctness:** Each update on the sheet is correctly displayed by the correct user.
- **Visual Feedback:** A visual representation indicating an update by a subscriber that is not a part of the global sheet yet.
- **GUI:** An area to place all the parameter requirements.
- ---
10. As a supplier, I want to store all of the subscriber’s updates for a specific spreadsheet, so that other subscribers and the publisher can view those updates. (Desirable Features - Medium)

**Acceptance Criteria** (conditions of satisfaction)

- **Correctness:** Each update on the sheet is correctly stored by the correct user.
- **Persistency:** Maintain a record of all updates and give them an ID.
- **Storage:** Ensure there is enough storage space for this.
- ---
11. As a subscriber, I want to see all the requested updates by my spreadsheets subscribers, so that I can approve which changes I want to make. (MVP - Medium)

**Acceptance Criteria** (conditions of satisfaction)

- **Correctness:** Each requested update is correct with the subscriber’s updates and the name of the subscriber.
- **Visual Representation:** Shows who updated the sheet and with what changes.
- ---
12. As a subscriber, I want to choose which requested updates are accepted, so that I can maintain the integrity of information in my spreadsheet. (MVP - Medium)

**Acceptance Criteria** (conditions of satisfaction)

- **Persistency:** Any updates approved will be synced with the actual spreadsheet and will be persistent after refreshing.
- **Choosing:** The publisher can choose which updates to approve and reject.
  - In the case of data conflicts, the most recently accepted update will be the one used as the global, updated sheet.
- **Visual Feedback:** On accepting an update, visual feedback will be provided.
- **Error Handling:** If an error occurs with saving the spreadsheet, an error message will be displayed and the attempted update will not through (latest successful update will remain).
- ---
13. As a subscriber, I want to request updates to be sent to a spreadsheet’s publisher, so that I can contribute to the project. (MVP - Medium)

**Acceptance Criteria** (conditions of satisfaction)

- **Visual Feedback:** On requesting an update, visual feedback will be provided.
- **Correctness:** All changes made on the client-end will be represented in the user’s requested update.
- **Status:** A visual representation of the status of the update (accepted, pending, declined).
- ---
14. As a supplier, I want to save spreadsheet updates, so that users can access their spreadsheet data at a later time. (MVP - High)

**Acceptance Criteria** (conditions of satisfaction)

- **Last Save:** When pulling updates from the server about a particular spreadsheet, the return value is the spreadsheet that was last saved by a user and was approved by the publisher.
- **Feedback**: When the data has been saved a visual indication should show that the save is successful / in progress.
- **Error Handling**: In case the save fails, the user should receive an error message explaining why the save was unsuccessful.
- ---
15. As a supplier, I want to choose my GUI’s theme, so that I have a nice color scheme to organize brands and outfits. (Bonus Features - Low)

**Acceptance Criteria** (conditions of satisfaction)

- **Feedback**: When a theme has been selected, there should be a visual indication that shows the selection was successful.
- **Persistency:** Whenever the user closes the application, the last selected theme should still be displayed.
- ---
16. As a supplier, I want to see my workers’ independent save histories, so that I know who is contributing and doing their job. (Bonus- Low)

**Acceptance Criteria** (conditions of satisfaction)

- **Correctness:** When checking the list of updates on a spreadsheet, each save history should display the correct user editor and the correct edits that were made.
- **Persistence:** Whenever the user closes the application, all of the user save histories should be stored and accessible on another open.
- ---
17. As a subscriber, I want to copy spreadsheets that my supplier has posted, so that I can become the supplier of my own copy and edit it further. (Bonus Features - Low)

**Acceptance Criteria** (conditions of satisfaction)

- **History Not Copied:** The copy will only copy over the sheet data, not the history
- **Different Copy Name:** The copy will be named “Copy of x”
- **New Copy Ownership:** The user is the owner of the new copy
- **Error Handling:** A visual indication will be shown if creating a copy fails.
- **User Sheets Updated:** The list of sheets the user owns will be updated.
- ---
18. As a supplier, I want to use functions and operations on my spreadsheet, so that my mathematical work is more efficient. (Desirable Features - Medium)

**Acceptance Criteria** (conditions of satisfaction)

- **Correctness:** When using a function on a column, the operations (such as ‘+’, ‘-’) should perform the correct work.
- **Error Handling:** When using operations, in the case of mismatched or incorrect typing, the user will be presented a small error message to indicate that.
- ---
19. As a subscriber, I want to set the privacy levels for my spreadsheets, so that I can choose which data I want to protect and which data I want help on. (Bonus Features - Low)

**Acceptance Criteria** (conditions of satisfaction)

- **Selection:** Selecting between private or public sheets.
- **GUI:** A visually represented area for users to choose between the two options
- **Visual Representation**: Indicating whether a sheet is private or public
- **Changing Privacy Level:** Giving users the chance to change their privacy level later on after making the spreadsheet.
- ---
20. As a subscriber, I want to give users the opportunity for privacy, so that they can maintain and protect their data. (Bonus Features - Low)

**Acceptance Criteria** (conditions of satisfaction)

- **User Sheets:** Subscribers cannot view private sheets from publishers.
- **Persistency:** The privacy level should remain the same after refreshing and logging in
- **Error Handling:** If a subscriber tries to access a private sheet, an error message will be shown.
