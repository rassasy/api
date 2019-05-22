SCRIPT_DIR=`cd $(dirname $0); pwd`
PROJECT_DIR=`cd "$SCRIPT_DIR/../../"; pwd`

cd $PROJECT_DIR

docker-compose -f resources/docker/api.yaml down
docker-compose -f resources/docker/storage.yaml down