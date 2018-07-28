package jp.ac.kyoto_su.tamadalab.heatman;

import java.awt.Color;
import java.util.Optional;

@FunctionalInterface
public interface ColorMapper {
    default Color map(Optional<Double> value) {
        return value.map(this::map).orElse(opaque());
    }

    Color map(double value);

    default Color opaque() {
        return new Color(0xffffffff);
    }
}
