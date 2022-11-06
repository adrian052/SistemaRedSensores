echo "Switching to branch master"
git checkout master

echo "Building app..."
npm run build

echo "Deploying files to server..."
scp -r build/* servidor3@148.217.202.124:/var/www/148.217.202.124/

echo "Done!"