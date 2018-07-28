package jp.ac.kyoto_su.tamadalab.heatman;

import java.io.IOException;

public class Main {
    public Main(String[] args) throws Exception {
        new Arguments(args).perform(this::perform);
    }

    private void perform(String dataFile, Arguments arguments) {
        try {
            HeatMapGenerator generator = new HeatMapGenerator(dataFile, arguments);
            generator.store(arguments.destination());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) throws Exception{
        new Main(args);
    }
}
