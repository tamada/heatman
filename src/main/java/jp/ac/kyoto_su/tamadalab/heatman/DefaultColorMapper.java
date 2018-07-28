package jp.ac.kyoto_su.tamadalab.heatman;

import java.awt.Color;

public class DefaultColorMapper implements ColorMapper {

    @Override
    public Color map(double value) {
        float result = (float)(((1 - value) * 240) / 360);
        return Color.getHSBColor(result, 1f, 1f);
    }
}
