package jp.ac.kyoto_su.tamadalab.heatman.entities;

import java.util.Arrays;
import java.util.stream.Stream;

public class XYLabels {
    private Labels xlabel = new Labels();
    private Labels ylabel = new Labels();

    public void parseXLabel(String line) {
        Arrays.stream(line.split(","))
        .skip(1)
        .forEach(xlabel::put);
    }

    public void putYLabel(String label) {
        ylabel.put(label);
    }

    public Stream<String> xlabels() {
        return xlabel.stream();
    }
    public Stream<String> ylabels() {
        return ylabel.stream();
    }
}
