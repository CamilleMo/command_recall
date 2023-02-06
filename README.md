# Command Recall

## Install
to install the cli:
`cargo install --git https://github.com/CamilleMo/command_recall`

## Configure token
To use this project, an openai token is needed.  
Get one at `https://platform.openai.com/account/api-keys`  

then configure the cli with your token:  
`command_recall configure --token <token>`  
or if you don't want to input your token in the terminal:  

`command_recall configure`  
It will create the config file without token  
then input your token manually in the config file.  

## Usage