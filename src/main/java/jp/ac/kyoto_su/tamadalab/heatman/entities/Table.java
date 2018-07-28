package jp.ac.kyoto_su.tamadalab.heatman.entities;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Table<T> {
    private List<List<T>> table = new ArrayList<List<T>>();

    public void putNextRow(Stream<T> columns) {
        table.add(columns.collect(Collectors.toList()));
    }

    public Optional<T> get(int x, int y) {
        return find(y, table).flatMap(list -> find(x, list));
    }

    private <K> Optional<K> find(int index, List<K> list) {
        if(index >= 0 && index < list.size())
            return Optional.of(list.get(index));
        return Optional.empty();
    }

    public int row() {
        return table.size();
    }

    public int column() {
        return table.stream()
                .mapToInt(list -> list.size())
                .max().orElse(0);
    }

    public int column(int row) {
        return table.get(row).size();
    }
}
