package jp.ac.kyoto_su.tamadalab.heatman;

import java.io.IOException;

public class Main {
    public Main(String[] args) throws Exception {
        Arguments arguments = new Arguments(args);
        arguments.perform(this::perform);
        if(arguments.isOutputScaler()) {
            outputScaler(arguments);
        }
    }

    private void perform(String dataFile, Arguments arguments) {
        try {
            HeatMapGenerator generator = new HeatMapGenerator(dataFile, arguments);
            generator.store(arguments.destination());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private void outputScaler(Arguments arguments) {
        try{
            HeatMapScaler scaler = new HeatMapScaler(arguments);
            scaler.store(arguments.destination());
        } catch(IOException e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) throws Exception{
        new Main(args);
    }
}
