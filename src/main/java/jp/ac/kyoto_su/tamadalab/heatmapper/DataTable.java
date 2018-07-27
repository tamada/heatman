package jp.ac.kyoto_su.tamadalab.heatmapper;

import java.awt.Dimension;
import java.io.BufferedReader;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.Optional;
import java.util.stream.Stream;

public class DataTable {
    private XYLabels labels = new XYLabels();
    private Table<Double> table = new Table<>();

    public DataTable(String dataFile) {
        parse(dataFile);
    }

    public Optional<Double> get(int i, int j) {
        return table.get(i, j);
    }

    public Dimension size() {
        return new Dimension(table.column(), table.row());
    }

    private void parse(String file) {
        try {
            parseImpl(Paths.get(file));
        } catch(IOException e) {
            e.printStackTrace();
        }
    }

    private void parseImpl(Path file) throws IOException {
        try(BufferedReader in = Files.newBufferedReader(file)){
            labels.parseXLabel(in.readLine());
            while(read(in.readLine()));
        }
    }

    private boolean read(String line) {
        if(line != null) {
            String[] items = line.split(",");
            labels.putYLabel(items[0]);
            table.putNextRow(convert(items));
        }
        return line != null;
    }

    private Stream<Double> convert(String[] items) {
        return Arrays.stream(items)
                .skip(1)
                .map(number -> Double.valueOf(number));
        
    }
}
