package cellularAutomata.utils;
import javafx.scene.paint.Color;
import javafx.scene.image.Image;
import javafx.scene.image.WritableImage;
import javafx.scene.image.PixelReader;
import javafx.scene.image.PixelWriter;
import java.util.ArrayList;
import java.util.*;
public interface ICellularGrid<T>{
	public Image getNextState(PixelReader read);
	public Image getCurrentState();
}
