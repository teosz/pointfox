for file in ../models/*
do
 if [ $file != "../models/models.rs" ]
 then
    echo "Running: "  $file
    sh ../models/.runr.sh $file
 fi
done
