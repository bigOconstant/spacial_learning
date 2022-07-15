# Spacial repetition web app

This repository is https://www.youtube.com/watch?v=2oRXK8FG1_4 to house a web based mobile first spacial repetition flash card learning application.

The ultimate goal is to make it easily self hostable and support some kind of federation.

What is federated is still up for debate. Flash cards could federate and we could add comments under 
a deck so we can have some kind of learning social network. This could be used in classes.
A teacher could host a instance and pull decks from outside instances somehow. Students could
comment on what is missing. 

We could also add comments under the card itself so the teacher could answer questions.

Ultimately I think it should support import and export of anki decks but without being limited
by it's format. Which is pretty old now. 

It should be a pure rust no javascript needed web application and every action should also be supported
in api. 

This is so that it is easy to host with a single service.

## developement

A docker-compose vscode environment is provide to make development easier. Start with 

`docker-compose -f Docker/Development/docker-compose.yml up `

And vscode should offer the option of openeing in a container. From there you can debug the rust program.
