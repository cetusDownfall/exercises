import javafx.application.Application;
import javafx.stage.Stage;
import javafx.scene.Scene;
import javafx.scene.Group;
import javafx.scene.layout.*;
import javafx.scene.control.*;
import javafx.event.EventHandler;
import javafx.event.*;
import javafx.scene.paint.Color;
import javafx.scene.image.WritableImage;
import javafx.scene.image.Image;
import javafx.scene.image.PixelReader;
import javafx.scene.image.PixelWriter;
import javafx.scene.image.ImageView;
import javafx.scene.input.KeyEvent;
import javafx.animation.Animation;
import javafx.animation.AnimationTimer;
import javafx.animation.Timeline;
import javafx.animation.KeyFrame;
import javafx.animation.*;
import javafx.util.Duration;
import java.util.HashMap;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.*;

public class WireTest extends Application{
	public static void main(String[] args){launch(args);}
	private enum State{DEAD(Color.GRAY),WIRE(Color.ORANGE),HEAD(Color.WHITE),TAIL(Color.BLUE);
		private Color c;
		State(Color olor){c=olor;}Color getColor(){return c;}
	}
	private class Dot{
		private int[] pos=new int[2];private int side;private State state = State.DEAD;
		public Dot(int r,int c,int si,State st){pos[0]=r;pos[1]=c;side=si;state=st;}
		public int getPos(int i){return pos[i];}public int getSide(){return side;}
		public void setPos(int...s){pos[0]=s[0];pos[1]=s[1];}public void setState(State s){state=s;}
		public Color getColor(){return state.getColor();}public State getState(){return state;}
		public int[][] getAdjPos(){
			int[][] out = new int[side*side][2];
			for(int i=0;i<side;i++){
				for(int j=0;j<side;j++){
					out[i*side+j][0]=i;
					out[i*side+j][1]=j;
				}
			}
			return out;
		}
	}
	private class Grid{
		private Dot[][] coords;
		private HashSet<Dot> dots=new HashSet<Dot>();
		private int length,cellCt,size;
		private WritableImage cells;
		private PixelReader reader;
		private PixelWriter writer;
		public Grid(int l, int s){
			length=l;
			size=s;
			cells=new WritableImage(length,length);
			reader=cells.getPixelReader();writer=cells.getPixelWriter();cellCt=length/size;coords=new Dot[cellCt][cellCt];
			for(int i=0;i<coords.length;i++){for(int j=0;j<coords[i].length;j++){coords[i][j]=new Dot(i,j,size,State.DEAD);addDot(coords[i][j]);}}paintByCoords();}
		public void paintByCoords(){
			for(int i=0;i<cellCt;i++){
				for(int j=0;j<cellCt;j++){
					if(coords[i][j]!=null){
						for(int ai:adjPos(i)){
							for(int aj:adjPos(j)){
								writer.setColor(aj,ai,coords[i][j].getColor());
							}
						}
					}
				}
			}
		}
		public void paintByDots(){
			for(Dot d:dots){
				for(int ai:adjPos(d.getPos(0))){
					for(int aj:adjPos(d.getPos(1))){
						writer.setColor(aj,ai,d.getColor());
					}
				}
			}
		}
		public int[] adjPos(int p){
			int[] value=new int[size];
			for(int i=0;i<size;i++){
				value[i]=(size*p)+i;
			}
			return value;
		}
		public int getLength(){return length;}public int getSize(){return size;}public void setLength(int n){length=n;}public void setSize(int n){size=n;}
		public void addDot(Dot d){coords[d.getPos(0)][d.getPos(1)]=d;if(d.getState()!=State.DEAD) dots.add(d);else dots.remove(d);}
		public void setState(int r,int c,State s){coords[r][c].setState(s);}
		public Color getColor(int i,int j){return coords[i][j].getColor();}
		public int headCt(int r,int c){int ct=0;for(int i=-1;i<=1;i++){for(int j=-1;j<=1;j++){if(coords[r+i][c+j]!=null&&!(i==0&&j==0)&&coords[r+i][j+j].getState().equals(State.HEAD)){ct++;}}}return ct;}
		public State getState(int r, int c){return coords[r][c].getState();}
		public State nextState(int r,int c){State outState=State.DEAD;switch(coords[r][c].getState()){case HEAD: outState=State.TAIL;break;case TAIL: outState=State.WIRE;break;
			case WIRE: outState=(headCt(r,c)==1||headCt(r,c)==2)?State.HEAD:State.WIRE;break;}return outState;}
		public void nextGridState(){
			HashSet<Dot> next=new HashSet<Dot>();
			for(Dot d:dots){
				int i=d.getPos(0);
				int j=d.getPos(1);
				next.add(new Dot(i,j,size,nextState(i,j)));
			}
			dots.addAll(next);
			for(Dot d:dots){
				int i=d.getPos(0);
				int j=d.getPos(1);
				coords[i][j]=d;
			}
			paintByDots();
		}
		public WritableImage toImage(){return cells;}
		public Dot[][] getContents(){return coords;}
		public HashSet<Dot> getWires(){return dots;}
	}
	private class Pen extends Dot{
		private boolean down = true,moved = false,fine = false;
		private boolean[] pushed = new boolean[]{false,false,false,false};
		private int[] netV = new int[]{0,0};
		private int[] coordPos = new int[]{0,0};
		private Grid page;
		private int radius,bRadius,cellMid;
		public Pen(Grid grid){this(grid.getLength()/2+1,grid.getLength()/2+1,grid);}
		public Pen(int posR, int posC, Grid grid){super(posR,posC,grid.getSize(),State.HEAD);
			page=grid;radius=page.getSize()/4;bRadius=radius+(getSide()/8);
			cellMid=grid.getSize()/2;
			coordPos[0] = grid.getContents().length/2;coordPos[1] = grid.getContents().length/2;
			center();
		}
		public void center(){setPos(coordPos[0]*page.getSize()+cellMid,coordPos[1]*page.getSize()+cellMid);}
		public boolean isDown(){return down;}
		public HashSet<Dot> getCell(){
			HashSet<Dot> cts=new HashSet<Dot>();
			for(int i : page.adjPos(coordPos[0])){
				for(int j : page.adjPos(coordPos[1])){
					if(Math.abs(getPos(0)-i)<radius&&Math.abs(getPos(1)-j)<radius){
						cts.add(new Dot(i,j,1,page.getContents()[getCoordPos(0)][getCoordPos(1)].getState()));
					}
				}
			}
					
			return cts;
		}
		public int getCoordPos(int i){return coordPos[i];} 
		public HashSet<Dot> getBorder(){
			HashSet<Dot> cts = new HashSet<Dot>();
			for(Dot d:getCell()){
				if(Math.abs(getPos(0)-d.getPos(0))<bRadius&&Math.abs(getPos(1)-d.getPos(1))<bRadius){
					cts.add(d);}}return cts;}
		public void move(){
			center();
			boolean canMove = !moved||!isFine();
			for(int i=0;i<2;i++){
				canMove=((coordPos[i]==0&&netV[i]<0) && (coordPos[i]==page.getContents().length-1)&&netV[i]>0);
				netV[i]*=canMove?1:0;
			}
			moved=!(netV[0]==0&&netV[1]==0);
			setPos(getPos(0)+netV[0]*(getSide()+coordPos[0]),getPos(1)+netV[1]*(getSide()+coordPos[1]));
			center();
		}
		public void ready(){moved=false;}
		public boolean hasMoved(){return moved;}
		public boolean isFine(){return fine;}
		public void setPrecise(boolean v){fine=v;}
		public void stop(){netV[0] = 0;netV[1] = 1;}
		public void push(int d){
			int adjD=((d%4)+4)%4;
			int ind = adjD%2;
			int sign = adjD>0&&adjD<3?1:-1;
			pushed[adjD] = true;
			if(pushed[ind]&&pushed[ind+2]){
				netV[ind]=0;
				pushed[ind]=false;
				pushed[ind+2]=false;
			}
			netV[ind]=sign;
		}
	}
	public void cycle(Grid grid,ImageView iv){grid.nextGridState();iv.setImage(grid.toImage());}
	@Override
	public void start(Stage stage){
		final int ZOOM = 9;
		Grid grid = new Grid(720,45);
		Pen pen = new Pen(grid);
		BorderPane bp = new BorderPane();
		WritableImage tamp = grid.toImage();
		PixelWriter tmp = tamp.getPixelWriter();
		ImageView iv = new ImageView(tamp);bp.setCenter(iv);
		iv.setFitWidth(grid.getLength());
		iv.setFitHeight(grid.getLength());
		Button stBtn = new Button("Start");Button endBtn = new Button("End");VBox ctrl = new VBox(stBtn,endBtn);bp.setLeft(ctrl);
		Scene scene = new Scene(bp, 1024, 860, Color.BLACK);
		stage.setScene(scene);
		Timeline tmln = new Timeline();
		tmln.getKeyFrames().clear();
		tmln.setCycleCount(Timeline.INDEFINITE);
		tmln.getKeyFrames().add(new KeyFrame(Duration.millis(0.017),new EventHandler<ActionEvent>(){
			@Override
			public void handle(ActionEvent ae){
				pen.move();
				if((!pen.isDown())&&pen.hasMoved()){pen.setState(grid.getState(pen.getCoordPos(0),pen.getCoordPos(1)));}
				if(pen.isDown()&&pen.hasMoved()){
					grid.setState(pen.getCoordPos(0),pen.getCoordPos(1),pen.getState());
					grid.paintByCoords();
				}
				for(Dot d:pen.getCell()){tmp.setColor(d.getPos(1),d.getPos(0),d.getColor().invert());}
				for(Dot d:pen.getBorder()){tmp.setColor(d.getPos(1),d.getPos(0),d.getColor().brighter());}
			}}));tmln.play();
/*		AnimationTimer drawing = new AnimationTimer(){
			@Override
			public void handle(long now){
				pen.move();
				if((!pen.isDown())&&pen.hasMoved()){pen.setState(grid.getState(pen.getCoordPos(0),pen.getCoordPos(1)));}
				if(pen.isDown()&&pen.hasMoved()){
					grid.setState(pen.getCoordPos(0),pen.getCoordPos(1),pen.getState());
					grid.paintByCoords();
				}
				for(Dot d:pen.getCell()){tmp.setColor(d.getPos(1),d.getPos(0),d.getColor().invert());}
				for(Dot d:pen.getBorder()){tmp.setColor(d.getPos(1),d.getPos(0),d.getColor().brighter());}
			}};*/
		stBtn.setOnAction(new EventHandler<ActionEvent>(){
			@Override
			public void handle(ActionEvent e){
				tmln.stop();
				tmln.getKeyFrames().clear();
				tmln.getKeyFrames().add(new KeyFrame(Duration.millis(0.017),new EventHandler<ActionEvent>(){
					@Override
					public void handle(ActionEvent ae){
						cycle(grid,iv);
						iv.setImage(grid.toImage());
						}
					}));
					tmln.play();
				}
			});
		endBtn.setOnAction(new EventHandler<ActionEvent>(){
			@Override
			public void handle(ActionEvent e){
				tmln.stop();
				tmln.getKeyFrames().clear();
			}});
		scene.setOnKeyPressed(new EventHandler<KeyEvent>(){
			@Override
			public void handle(KeyEvent e){
				switch(e.getCode()){
					case H: pen.push(3);break;
					case J: pen.push(2);break;
					case K: pen.push(0);break;
					case L: pen.push(1);break;
					case SHIFT: pen.setPrecise(true);break;
				}}});
		scene.setOnKeyReleased(new EventHandler<KeyEvent>(){
			@Override
			public void handle(KeyEvent e){
				switch(e.getCode()){
					case H: pen.push(1);break;
					case J: pen.push(0);break;
					case K: pen.push(2);break;
					case L: pen.push(3);break;
					case SHIFT: pen.setPrecise(false);break;
				}
			}
		});
		stage.show();
	}
}
