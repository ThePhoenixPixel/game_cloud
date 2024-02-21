# installer für die game_cloud

# check ob er admin ist
if [ "$EUID" -ne 0 ]; then
  echo "Fehler: Sie müssen als Administrator (root) angemeldet sein, um auf 'apt' zuzugreifen"
  echo "gebe in terminal (cmd) sudo installer.sh ein"
  exit 1
fi

echo "So dann kann es loslegen"
echo "Werbung bekommste zum schluss :)))"
echo "Kein Stress ich helf dir bei der Installation von der GameCloud"
echo "Wir machen das gemeinsam Schritt für Schritt"

echo "Erstmal klassische Frage Stimmst du der eula von Minecarft (Mojang) zu (y/N)"
read eula

if [ "$eula" != "y" ]; then
  echo "EULA muss zugestimmt werden"
  exit 1
fi

read -p "Bitte gebe zuerst die ip addresse an auf die deine backend server (game_server) gebindet werden sollen" server-ip

echo "So das wäre schon alles wir updateten jetzt mal alles und installiren uns n bisschen was"
echo "Könnte kurz dauern Lehn dich zurück und geniese das Gefühl als ob du ein krasser häcker wärst"

# start the installation progress

apt update
apt upgrade -y
apt autoremove

#install requier
apt install screen -y
apt install wget -y

#create the folder
mkdir /home/game_cloud/

cd /home/game_cloud/

#download the cloud
wget https://download.codergames.de/game_cloud/v0.1/game_cloud


