package jp.ac.kyoto_su.tamadalab.heatman.entities;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Stream;

public class Labels {
    private List<String> labels = new ArrayList<>();

    public void put(String item) {
        labels.add(item);
    }

    public int size() {
        return labels.size();
    }

    public String label(int index) {
        return labels.get(index);
    }

    public Stream<String> stream() {
        return labels.stream();
    }
}
