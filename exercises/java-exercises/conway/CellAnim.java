package cellularAutomata.conway;
import cellularAutomata.utils.*;
import javafx.scene.image.WritableImage;
public class CellAnim extends AbstractCellularAnimation{
	private Cells c;
	private static final int GRIDSIZE = 180;
	@Override
	public void setupAutomaton(){
		c = new Cells(GRIDSIZE,30);
	}
	@Override
	public WritableImage getCurrentImage(){
		return c.getCurrentImage();
	}
	@Override
	public WritableImage getNextImage(){
		c = new Cells(c.getNextImage());
		return c.getCurrentImage();
	}
	public static void main(String[] args){
		launch(args);
	}
}
