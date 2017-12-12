package cellularAutomata.wireworld;
import cellularAutomata.utils.*;
import javafx.animation.Animation;
import javafx.animation.Timeline;
import javafx.util.Duration;
import javafx.animation.KeyFrame;
import javafx.scene.image.WritableImage;
import javafx.scene.image.PixelWriter;
import javafx.scene.layout.BorderPane;
import javafx.scene.control.TextArea;
import cellularAutomata.wireworld.Wire.CellType;
import javafx.scene.*;
import javafx.scene.input.KeyEvent;
import javafx.scene.layout.VBox;
import javafx.scene.Node;
import javafx.scene.image.*;
import javafx.scene.layout.*;
import javafx.scene.paint.Color;
import javafx.event.EventHandler;
import javafx.event.*;
import javafx.beans.value.*;
import javafx.beans.*;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.Set;
import java.util.*;
import java.util.Scanner;
public class WireAnim extends AbstractCellularAnimation{
	private WireDrawPoint cur;
	private int dim;
	private PixelWriter startingPen;
	private Timeline tmln = new Timeline();
	private Wire[][] wireMap;
	private HashSet<Wire> wireHash = new HashSet<Wire>();
	private void refreshStates(){
		wireHash.clear();
		for (Wire[] row : wireMap){
			for (Wire wr : row){
				wr.addTo(wireMap,wireHash);
			}
		}
	}
	@Override
	public WritableImage getNextImage(){
		WritableImage g = new WritableImage(getCurrentImage().getPixelReader(), wireMap.length,wireMap[0].length);
		PixelWriter pen = g.getPixelWriter();
		for (Wire wr : wireHash){
			pen.setColor(wr.getPoint().getPos(1), wr.getPoint().getPos(0), wr.getNextState());
		}
		for (Wire wr : wireHash){
			wr.update();
		}
		return g;
	}
	@Override
	public WritableImage getCurrentImage(){
		WritableImage g = new WritableImage(wireMap.length,wireMap[0].length);
		PixelWriter pen = g.getPixelWriter();
		for (int i = 0; i < wireMap.length; i++){
			for (int j = 0; j < wireMap[i].length; j++){
				pen.setColor(j,i,wireMap[i][j].getCurrentState());	
			}
		}
		return g;
	}
	public Wire[][] getWires(Image img){
		PixelReader getter = img.getPixelReader();
		Wire[][] outW = new Wire[(int)(img.getHeight()+.5)][(int)(img.getWidth()+.5)];
		for (int i = 0; i < outW.length; i++){
			for (int j = 0; j < outW[i].length; j++){
				Color c = getter.getColor(j,i);
				CellType t = CellType.DEAD;
				if (c.equals(CellType.WIRE.getCurrentState())){
					t=CellType.WIRE;
				}else if (c.equals(CellType.HEAD.getCurrentState())){
					t=CellType.HEAD;
				}else if (c.equals(CellType.TAIL.getCurrentState())){
					t=CellType.TAIL;
				}else{
					t = CellType.DEAD;
				}
				outW[i][j] = new Wire(i,j,t);
			}
		}
		return outW;
	}
	@Override
	public void setupAutomaton(ImageView startImv){
		/*Image pre = startImv.getImage();
		Wire[][] tmp = getWires(pre);
		for (int i = 0; i < tmp.length; i++){
			for (int j = 0; j < tmp[i].length; j++){
				wireMap[i][j] = tmp[i][j];
			}
		}*/
		refreshStates();
		startImv.setImage(getCurrentImage());
		tmln.stop();
		tmln.getKeyFrames().clear();
		cur.stop();
	}

	@Override
	public void prepareAuto(WritableImage stImg,ImageView imv){
		dim = (int) (stImg.getHeight()+.5);
		imv.setImage(stImg);
		PixelWriter startingPen = stImg.getPixelWriter();
		if (wireMap == null){
			wireMap = new Wire[dim][dim];
		}
		for (int i = 0; i < wireMap.length; i++){
			for (int j = 0; j < wireMap[i].length; j++){
				wireMap[i][j] = new Wire(i,j,CellType.DEAD);
			}
		}
		Wire[][] temp = getWires(stImg);
		for (int i = 0; i < wireMap.length; i++){
			for (int j = 0; j < wireMap[i].length; j++){
				wireMap[i][j] = temp[i][j];
				startingPen.setColor(j,i,wireMap[i][j].getCurrentState());
			}
		}
		cur = new WireDrawPoint(wireMap.length,wireMap[0].length);
		tmln.setCycleCount(Timeline.INDEFINITE);
		KeyFrame frm = new KeyFrame(
		Duration.seconds(0.017),
		new EventHandler<ActionEvent>(){
			public void handle(ActionEvent ae){
				if (cur.getDraw()){
					wireMap[cur.getPos(0)][cur.getPos(1)].setCellType(cur.getType());
				}
				for (int i = -dim/180; i <= dim/180; i++){
					for (int j = -dim/180; j <= dim/180; j++){
						if (cur.getPos(0) + i >= 0 && cur.getPos(0) + i < dim && cur.getPos(1) + j >= 0 && cur.getPos(1) + j < dim){
							startingPen.setColor(cur.getPos(1)+j,cur.getPos(0)+i,wireMap[cur.getPos(0)+i][cur.getPos(1)+j].getCurrentState());
						}
					}
				}
				cur.move();
				for (int i = -dim/180; i <= dim/180; i++){
					for (int j = -dim/180; j <= dim/180; j++){
						if (cur.getPos(0) + i >= 0 && cur.getPos(0) + i < dim && cur.getPos(1) + j >= 0 && cur.getPos(1) + j < dim){
							int a = cur.getPos(1)+j;
							int b = cur.getPos(0)+i;
							startingPen.setColor(a,b,cur.getDraw()?wireMap[b][a].getCurrentState():wireMap[b][a].getCurrentState().invert());
							if (cur.getDraw()||cur.getSlow()){
								if (i == 0 || j == 0){
									startingPen.setColor(a,b,wireMap[b][a].getCurrentState().darker().invert());
								}
							}
						}
					}
				}
				if (!cur.getDraw()){
					startingPen.setColor(cur.getPos(1),cur.getPos(0),wireMap[cur.getPos(0)][cur.getPos(1)].getCurrentState().invert());
				}else{
					startingPen.setColor(cur.getPos(1),cur.getPos(0),cur.getType().getCurrentState());
				}
			}
		});
		tmln.getKeyFrames().clear();
		tmln.getKeyFrames().add(frm);
		tmln.play();
	}
	@Override
	public void setupPanel(Scene s){
		s.setOnKeyPressed(new EventHandler<KeyEvent>(){
			@Override
			public void handle(KeyEvent ke){
				boolean shift = false;
				switch (ke.getCode()){
					case H: cur.push('l',1);break;
					case J: cur.push('d',1);break;
					case K: cur.push('u',1);break;
					case L: cur.push('r',1);break;
					case W: cur.setType(CellType.WIRE);cur.setDraw(true);break;
					case A: cur.setType(CellType.TAIL);cur.setDraw(true);break;
					case S: cur.setType(CellType.HEAD);cur.setDraw(true);break;
					case D: cur.setType(CellType.DEAD);cur.setDraw(true);break;
					case SHIFT: cur.setSlow();shift = true;break;
				}
			}
		});
		s.setOnKeyReleased(new EventHandler<KeyEvent>(){
			@Override
			public void handle(KeyEvent ke){
				switch (ke.getCode()){
					case H: cur.push('l',0);break;
					case J: cur.push('d',0);break;
					case K: cur.push('u',0);break;
					case L: cur.push('r',0);break;
					case W:
					case A:
					case S:
					case D: cur.setType(wireMap[cur.getPos(0)][cur.getPos(1)].getCellType());cur.setDraw(false);break;
					case SHIFT: cur.setFast();break;
				}
			}
		});
	}
	public static void main(String[] args){
		launch(args);
	}
}
