package jp.ac.kyoto_su.tamadalab.heatman;

import java.util.HashMap;
import java.util.Map;

public class ColorMapperBuilder {
    private static final ColorMapperBuilder INSTANCE = new ColorMapperBuilder();

    private Map<Boolean, ColorMapper> builders = new HashMap<>();

    private ColorMapperBuilder() {
        builders.put(true, new GrayScaleColorMapper());
        builders.put(false, new DefaultColorMapper());
    }

    public static ColorMapper build(boolean flag) {
        return INSTANCE.builders.get(flag);
    }
}
