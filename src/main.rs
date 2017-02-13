

use std::io;

extern crate rand;

use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: String
}

impl Point {
    fn new(x: f64, y: f64, z: String) -> Point {
        Point { x: x, y: y, z: z}
    }
}


struct GameTable {
    plays: Vec<Point>,
}

impl GameTable{

	//functions to write
	//check if unbounded three (to guard against players unbounded three from computer control)
	//and also to make it's move if it cant win and doesnt have to guard against an immediate player win




	fn findunboundedchain(&mut self, playerorcomputer: String) -> (f64){

		//the counter variable should have fixed the problem of not skipping checking the other players values
		//the infinite loop under the condition that the fouraway variable is true is being caused
		//by some issue in computercontrol.

    	let mut longestunboundedchain = 0.0;
    	let mut longestunboundedchainthiscycle = 0.0;
    	let mut longestunboundedchaindummy = 0.0;
    	let mut sidesunbounded = 0.0 as f64;
    	let mut unboundedlefttotest = &self.plays[0];
    	let mut unboundedrighttotest = &self.plays[0];
    	let mut longestunboundedleft = &self.plays[0];
    	let mut longestunboundedright = &self.plays[0];
    	let mut notsidesunbouded = 0.0 as f64;

    
    	let mut longestreturnvaluedummy = 0.0;
    	let mut longestreturnvalue = 0.0;



		let mut distfloat1 = (2usize as f64).sqrt();
		let mut distfloat2 = (8usize as f64).sqrt();
		let mut distfloat3 = (18usize as f64).sqrt();
		let mut distfloat4 = (32usize as f64).sqrt();

		let mut calculation = 0.0;
		let mut calculationdummy = 0.0;
		let mut calculatedistance = 0.0;
		let mut calculatedistancedummy = 0.0;
		let mut countmaxlengthdummy = 0.0 as f64;

		let mut firstpointx = 0.0;
		let mut firstpointy = 0.0;

		let mut secondpointx = 0.0;
		let mut secondpointy = 0.0;

		let mut dummypointx = 0.0;
		let mut dummypointy = 0.0;

		let mut countmaxlength = 0.0 as f64;

		for iterate in 0..(self.plays.len()){

			if self.plays[iterate].z == playerorcomputer{
				
				unboundedrighttotest = &self.plays[iterate];
				
				firstpointx = self.plays[iterate].x;
				firstpointy = self.plays[iterate].y;

				dummypointx = firstpointx;
				dummypointy = firstpointy;
			}


			longestunboundedchainthiscycle = 0.0;

		//	println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&");

			countmaxlength = 0.0;

			for iteratesecond in 0..(self.plays.len()){

				//if iteratesecond >= iterate{

					unboundedlefttotest = &self.plays[iteratesecond];


					if unboundedrighttotest.z == unboundedlefttotest.z{
					
						firstpointx = unboundedrighttotest.x;
						firstpointy = unboundedrighttotest.y;

						secondpointx = unboundedlefttotest.x;
						secondpointy = unboundedlefttotest.y;
					
						calculation = ((firstpointx - secondpointx)*(firstpointx - secondpointx) + (firstpointy - secondpointy)*(firstpointy - secondpointy));
						calculationdummy = ((dummypointx - secondpointx)*(dummypointx - secondpointx) + (dummypointy - secondpointy)*(dummypointy - secondpointy));

						calculatedistance = (calculation as f64).sqrt();
						calculatedistancedummy = (calculationdummy as f64).sqrt();

					}else {
				
						calculatedistancedummy = 0.0;
				
					}

					let mut twosquareroot = 2.0;

					if calculatedistancedummy == 1.0 || calculatedistancedummy == (twosquareroot as f64).sqrt(){
				

					//println!("before comparison operation");
					//println!("countmaxlength {:?}", countmaxlength);
					//println!("countmaxlengthdummy {:?}", countmaxlengthdummy);
					
						if calculatedistance == 1.0 || calculatedistance == (twosquareroot as f64).sqrt(){
							countmaxlength += 1.0;
						}
					//println!("unboundedlefttotest {:?}", unboundedlefttotest);
					//println!("unboundedrighttotest {:?}", unboundedrighttotest);
					//println!("countmaxlength {:?}", countmaxlength);

						dummypointx = unboundedrighttotest.x;
						dummypointy = unboundedrighttotest.y;
					}


			//		println!("_++_++_++_+__+__++_++_++_+__+__++_++_++_+__+__++_++_++_+__+_+_");
			//		println!("countmaxlength {:?}", countmaxlength);
			//		println!("countmaxlengthdummy {:?}", countmaxlengthdummy);
			//		println!("unboundedrighttotest {:?}", unboundedrighttotest);
			//		println!("unboundedlefttotest {:?}", unboundedlefttotest);
			//		println!("calculatedistance {:?}", calculatedistance);
			//		println!("calculatedistancedummy {:?}", calculatedistancedummy);

					if countmaxlength>=countmaxlengthdummy && unboundedlefttotest.z == playerorcomputer && calculatedistancedummy == 1.0{
				//if (oneaway == true && oneawayprevious == false) || (twoaway == true && twoawayprevious == false) || (threeaway == true && threeawayprevious == false) || (fouraway == true && fourawayprevious == false){

						if countmaxlength > longestunboundedchainthiscycle{
							longestunboundedchainthiscycle = countmaxlength;
						}

			//			println!("longestunboundedchainthiscycle {:?}", longestunboundedchainthiscycle);




						if longestunboundedchainthiscycle > longestunboundedchain{
							longestunboundedchain = longestunboundedchainthiscycle;
						}


					}

				//}
			}

		}

		longestunboundedchain += 1.0;

		return (longestunboundedchain);

	}




	fn computercontrol(&mut self, computersteals: u32, playersteals: u32) -> u32{




		//way to fix this: see if the player can win in TWO moves ahead






		//ACHTUNG ACHTUNG

		//I need to make sure here that I can look ahead for TWO moves of the player
		//because of the condition in which there is a three that is unbounded on either side


		//aditional note: Ideally the computer should not want to go anywhere such that the player
		//can capture the computers pieces on the next turn.




		
		//CHECK IF COMPUTER CAN WIN THIS TURN
		//*****************************************************
		//1. by capturing fifth pair
			//is there four steals already?
				//if yes loop through game GameTable
					//does steal increment by one?
						//if yes computer goes here
						//if no go to two
				//if no go to two
		//2. by five in a row
			//loop through game table
			//if yes computer goes here
			//if no "CHECK IF PLAYER CAN WIN NEXT TURN"
		//*****************************************************



		//1. by capturing fifth pair
			//is there four steals already?


		let mut computercanwinthisturn = 0;
		let mut computerwins = 0;
		let mut computerwinsstring = String::new();

			'outer: for i in 0i32..31 { //B
				for j in 0i32..31{ //C

					let mut newpoint = Point {
					    x: i as f64,
					    y: j as f64,
					    z: "computer".to_string(),
					};

					self.plays.push(newpoint);
        			computerwins = self.updatesteals() + computersteals;
        			let mut computerwinsstring = self.check4fiveinarow();

        			if computerwins == 5{ //D
        				computercanwinthisturn = 1;
        				break 'outer;
        			} //D

        			if computerwinsstring == "computer"{ //H
        				computercanwinthisturn = 1;
        				break 'outer;
        			} //H

        			let mut index = self.plays.len()-1;
					self.plays.remove(index);
				} //C
			} 
			

			//an unbounded 3 in a row by the player is as bad for the computer
			//as a win for the player on the next turn. So we can write the check for that here
			//(which I will do) or after the CHECK IF PLAYER CAN WIN NEXT TURN block.



	

		//CHECK IF PLAYER CAN WIN NEXT TURN 
		//*****************************************************
			//1. By five in a row
				//1.Put computer piece in coordinate in board and loop over game table
				//see if computer steals
					//if steals
						//player still win?
							//if yes the go to 2.
							//if no computer goes here
					//if no steal go to 2.
				//2. put piece where player can win
			//2. By capturing the fifth pair
			// put piece where player can win
		//*****************************************************

		let mut computerwinssteals = 0;
		let mut playercanwinthisturn = 0;
		let mut playerdoesntwinnextturn = String::new();



		if computercanwinthisturn == 0{ //I
			'outeri: for i in 0i32..31 { //J
				for j in 0i32..31{//k

					let mut newpoint = Point {
				        x: i as f64,
				        y: j as f64,
				        z: "player".to_string(),
				    }; 

				    self.plays.push(newpoint); //ALPHA


				    //LOOP HERE FOR ANOTHER PLAYER PUSH AND CHECK IF HE CAN WIN THEN BACK OUT WITH COMPUTER MOVES TO BLOCK






		        	let mut playerwinssteals = self.updatesteals() + playersteals;
		        	let mut playerwins5inrow = self.check4fiveinarow();
					        				
        			let mut index = self.plays.len()-1;
					self.plays.remove(index); //ALPHA


			//		if self.plays[index - 1].z == "computer" && self.plays[index - 2].z == "computer" {
			//			println!("We have encountered a problem!");
			//			println!("The problem occurred in the first outer loop");
			//			println!("the value of the second to last play {:?}", self.plays[index - 2]);
			//			println!("the value of the last play {:?}", self.plays[index - 1]);
			//			println!("i is {:?}", i);
			//			println!("j is {:?}", j);
			//			println!("playerwins5inrow {:?}", playerwins5inrow);
//
//					}


		        	if playerwinssteals == 5 || playerwins5inrow == "player"{ //L

		        		playercanwinthisturn = 1;

		        		'outerh: for h in 0i32..31{ //M
		        			for k in 0i32..31{ //N

		        			//	println!("here is the h value {:?}", h);
		        			//	println!("here is the k value {:?}", k);
		        			//	println!("here is the length of the move stack {:?}", self.plays.len());

								let mut newpoint = Point {
							        x: h as f64,
							        y: k as f64,
							        z: "computer".to_string(),
							    };

							    self.plays.push(newpoint); //BETA
					        	computerwinssteals = self.updatesteals();

					        	//*****************************************************************

					        	'outerl: for l in 0i32..31{ //O
					        		for m in 0i32..31{ //P

										let mut newpoint = Point {
									        x: l as f64,
									        y: m as f64,
									        z: "player".to_string(),
									    };

									    self.plays.push(newpoint); //GAMMA

							        	let mut playerwinsstealsinner = self.updatesteals() + playersteals;
							        	let mut playerwins5inrowinner = self.check4fiveinarow();

							        	if playerwinsstealsinner == 5 || playerwins5inrowinner == "player"{ //Q
							        		playerdoesntwinnextturn = "he does".to_string();
	        								let mut index = self.plays.len()-1;
											self.plays.remove(index); //GAMMA
							        		break 'outerl;
							        	} else{ //Q
							        		playerdoesntwinnextturn = "nope".to_string();
	        								let mut index = self.plays.len()-1;
											self.plays.remove(index); //GAMMA
							        	} //Q



				//						if self.plays[index - 1].z == "computer" && self.plays[index - 2].z == "computer" {
				//							println!("We have encountered a problem!");
				//							println!("The problem occurred in the first outer loop");
				//							println!("the value of the second to last play {:?}", self.plays[index - 1]);
				//							println!("the value of the last play {:?}", self.plays[index - 2]);
				//							println!("l is {:?}", l);
				//							println!("m is {:?}", m);
//
//										}




					        		} //P
					        	} //O

					        	//*****************************************************************


					        	if playerdoesntwinnextturn == "nope"{ //R
	        						let mut index = self.plays.len()-1;
									self.plays.remove(index); //BETA
					        		break 'outerh;
					        	} else {//R 
	        						let mut index = self.plays.len()-1;
									self.plays.remove(index); //BETA
								}


//								if self.plays[index - 1].z == "computer" && self.plays[index - 2].z == "computer" {
//									println!("We have encountered a problem!");
//									println!("The problem occurred in the second loop");
//									println!("the value of the second to last play {:?}", self.plays[index - 1]);
//									println!("the value of the last play {:?}", self.plays[index - 2]);
//									println!("h is {:?}", h);
//									println!("k is {:?}", k);
//
//								}



							}//N

		        		} //M
		        	}

		        }
		    }


		        	//cant have the comoputer go somewhere randomly if the player can win the next turn
		        	//rather than pretend to do their supposedly optimal strategy or the player will know that
		        	//the computer sees that they can win

		    if playerdoesntwinnextturn == "he does"{ //
		        'outer: loop{ //Z
		        	//println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
					let mut breakoutloop = String::new();
					let mut failedloop = "false";

					let mut rng = rand::thread_rng();
					let mut between = Range::new(0, 31);
					let mut pickx = between.ind_sample(&mut rng);
					rng = rand::thread_rng();
					between = Range::new(0, 31);
					let mut picky = between.ind_sample(&mut rng);

					for i in 0..(self.plays.len()){
						if self.plays[i].x == (pickx as f64) && self.plays[i].y == (picky as f64){
		        			//println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
							failedloop = "true";
							continue 'outer;
						}
					}

					if failedloop == "false"{ //X
						let mut newpoint = Point {
							x: pickx as f64,
							y: picky as f64,
							z: "computer".to_string(),
						};

						self.plays.push(newpoint);
						computerwinssteals = self.updatesteals();
							        
						break 'outer;
					} //X
				} //T
			} //S
		} //I
	



		//instead of otherwise going somewhere randomly
		//the computer might consider going wherever:
		//1. There is an unbounded three (given that it won't be taken on the next turn)
		//2. Wherever has the longest chain (given that it won't be taken on the next turn)



		//AND THEN
		//if there is no longest chain or unbounded three (that wouldn't lead to a capture)
		//THEN you can go somewhere randomly.



		//OTHERWISE GO SOMEWHERE RANDOMLY

		if playercanwinthisturn == 0{ //Y
			'outer: loop{ //Z
				let mut breakoutloop = String::new();
		        let mut failedloop = "false";

			    let mut rng = rand::thread_rng();
				let mut between = Range::new(0, 31);
				let mut pickx = between.ind_sample(&mut rng);
				rng = rand::thread_rng();
				between = Range::new(0, 31);
				let mut picky = between.ind_sample(&mut rng);

				let mut newpoint = Point {
					x: pickx as f64,
					y: picky as f64,
					z: "computer".to_string(),
				};

				for i in 0..(self.plays.len()-1){
					if self.plays[i].x == (pickx as f64) && self.plays[i].y == (picky as f64){
						failedloop = "true";
						break 'outer;
					}
				}

				if failedloop == "false"{ //DELTA


					let mut newpoint = Point {
				        x: pickx as f64,
				        y: picky as f64,
				        z: "computer".to_string(),
				    };

				    self.plays.push(newpoint);

				//	let self.plays[iterate].x = pickx;
				//	let self.plays[iterate].y = picky;
				//	let self.plays[iterate].z = "computer";
					break;
				} //DELTA
			} //Z
		} //Y


		return computerwinssteals;

	}





	fn updatesteals(&mut self) -> u32{

		let mut stealcountupdate = 0;

		let mut storepointindex1 = 99999.0;
		let mut storepointindex2 = 99999.0;
		let mut storepointindex3 = 99999.0;
		let mut storepointindex4 = 99999.0;

		let mut distfloat1 = (2usize as f64).sqrt();
		let mut distfloat2 = (8usize as f64).sqrt();
		let mut distfloat3 = (18usize as f64).sqrt();

		let mut calculation = 0.0 as f64;
		let mut calculationdummy = 0.0 as f64;
		let mut calculatedistance = 0.0 as f64;
		let mut calculatedistancedummy = 0.0 as f64;

		let mut oneaway = false;
		let mut twoaway = false;
		let mut threeaway = false;

		let mut firstpointx = 0.0 as f64;
		let mut firstpointy = 0.0 as f64;

		let mut secondpointx = 0.0 as f64;
		let mut secondpointy = 0.0 as f64;

		let mut dummypointx = 0.0 as f64;
		let mut dummypointy = 0.0 as f64;

		let mut iteratelength = self.plays.len();
		let mut iterate = 0;



		loop{


			if &self.plays[iterate].z == "player" || &self.plays[iterate].z == "computer"{
				firstpointx = self.plays[iterate].x;
				firstpointy = self.plays[iterate].y;

				dummypointx = firstpointx;
				dummypointy = firstpointy;
			}
			
			oneaway = false;
			twoaway = false;
			threeaway = false;
			storepointindex1 = iterate as f64;
			storepointindex2 = 99999.0;
			storepointindex3 = 99999.0;
			storepointindex4 = 99999.0;

			for iteratesecond in 0..(iteratelength){

					secondpointx = self.plays[iteratesecond].x;
					secondpointy = self.plays[iteratesecond].y;
					
					calculation = ((firstpointx - secondpointx)*(firstpointx - secondpointx) + (firstpointy - secondpointy)*(firstpointy - secondpointy));
					calculationdummy = ((dummypointx - secondpointx)*(dummypointx - secondpointx) + (dummypointy - secondpointy)*(dummypointy - secondpointy));

					calculatedistance = (calculation as f64).sqrt();
					calculatedistancedummy = (calculationdummy as f64).sqrt();
				

				if oneaway == false{


					if (calculatedistance == distfloat1 || calculatedistance == 1.0) && (calculatedistancedummy == distfloat1 || calculatedistancedummy == 1.0){ 



						if iterate != iteratesecond && &self.plays[iterate].z != &self.plays[iteratesecond].z {

							dummypointx = secondpointx;
							dummypointy = secondpointy;
							oneaway = true;
							storepointindex2 = iteratesecond as f64;

						}


					} else {

						storepointindex2 = 99999.0;
						oneaway = false;
						twoaway = false;
						threeaway = false;

					}
					
				}else if twoaway == false{



					if (calculatedistance == distfloat2 || calculatedistance == 2.0) && (calculatedistancedummy == distfloat1 || calculatedistancedummy == 1.0){
						

						

						if iterate != iteratesecond && &self.plays[iterate].z != &self.plays[iteratesecond].z {

							dummypointx = secondpointx;
							dummypointy = secondpointy;
							twoaway = true;
							storepointindex3 = iteratesecond as f64;
						
						}							


					} else {

						storepointindex3 = 99999.0;
						oneaway = false;
						twoaway = false;
						threeaway = false;
					}
					
				}else if threeaway == false{

					if (calculatedistance == distfloat3 || calculatedistance == 3.0) && (calculatedistancedummy == distfloat1 || calculatedistancedummy == 1.0){

						
						
						if iterate != iteratesecond && &self.plays[iterate].z == &self.plays[iteratesecond].z {



							dummypointx = secondpointx;
							dummypointy = secondpointy;
							threeaway = true;
							storepointindex4 = iteratesecond as f64;

						}


					} else {

						storepointindex4 = 99999.0;
						oneaway = false;
						twoaway = false;
						threeaway = false;

					}
					
				
				}else{
					println!("This statement should never be evaluated!");
					break;
				}


				if storepointindex1 != 99999.0 && storepointindex2 != 99999.0 && storepointindex3 != 99999.0 && storepointindex4 != 99999.0{

					let index = storepointindex2 as usize;

					self.plays.remove(index);
					self.plays.remove(index);
					iterate = 0;
					iteratelength = self.plays.len();
					stealcountupdate += 1;
					break;
				}


				
			}

			iterate+=1;
			if iterate == iteratelength{
				break;
			}
		
		}


		return stealcountupdate;




	}






	fn displaytable(&self) {
		let mut skipadot = String::new();
        println!("    0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30");
			for i in 0..31{
				if i <= 9 {
					print!("{:?}",i);
					print!("   ");
				} else {
					print!("{:?}",i);
					print!("  ");
				}
				skipadot = "dontskip".to_string();
				for j in 0..31{
					for iterate in 0..(self.plays.len()){
						if self.plays[iterate].x == i as f64 && self.plays[iterate].y == j as f64 {
							if self.plays[iterate].z == "player"{
								print!("O  ");
								skipadot = "skip".to_string();
								break;
							} else if self.plays[iterate].z == "computer"{
								print!("X  ");
								skipadot = "skip".to_string();
								break;
							} else {
								skipadot = "dontskip".to_string();
								break;
							}

						}

					}
					if skipadot == "dontskip"{
						print!(".  ");	
					} else{
						skipadot = "dontskip".to_string();
					}
				}
				println!("");
			}
	}

	fn check4fiveinarow(&self) -> String {

		//the counter variable should have fixed the problem of not skipping checking the other players values
		//the infinite loop under the condition that the fouraway variable is true is being caused
		//by some issue in computercontrol.






    	let mut returnwinner = String::new(); 
    	returnwinner = "no one yet".to_string();

		let mut distfloat1 = (2usize as f64).sqrt();
		let mut distfloat2 = (8usize as f64).sqrt();
		let mut distfloat3 = (18usize as f64).sqrt();
		let mut distfloat4 = (32usize as f64).sqrt();

		let mut calculation = 0.0;
		let mut calculationdummy = 0.0;
		let mut calculatedistance = 0.0;
		let mut calculatedistancedummy = 0.0;

		let mut oneaway = false;
		let mut twoaway = false;
		let mut threeaway = false;
		let mut fouraway = false;

		let mut firstpointx = 0.0;
		let mut firstpointy = 0.0;

		let mut secondpointx = 0.0;
		let mut secondpointy = 0.0;

		let mut dummypointx = 0.0;
		let mut dummypointy = 0.0;

		for iterate in 0..(self.plays.len()){

			if self.plays[iterate].z == "player" || self.plays[iterate].z == "computer"{
				firstpointx = self.plays[iterate].x;
				firstpointy = self.plays[iterate].y;

				dummypointx = firstpointx;
				dummypointy = firstpointy;
			}
			
			oneaway = false;
			twoaway = false;
			threeaway = false;
			fouraway = false;

			//println!("SELF PLAYS LENGTH {:?}", self.plays.len());


			let mut counter = 0;

			for iteratesecond in 0..(self.plays.len()){


//				if self.plays.len() == 10 {


//					println!("ITERATION {:?}", iterate);
//					println!("ITERATION THE SECOND {:?}", iteratesecond );
//					println!("FIRST POINT TO BE TESTED {:?}", self.plays[iterate]);
//					println!("SECOND SECOND POINT TO BE TESTED {:?}", self.plays[iteratesecond]);
//					println!("value of returnwinner in second for loop, {:?}", returnwinner);
//					println!("value of oneaway {:?}", oneaway);
//					println!("value of twoaway {:?}", twoaway);
//					println!("value of threeaway {:?}", threeaway);
//					println!("value of fouraway {:?}", fouraway);
//					println!("here's the print out of the entire stack {:?}", self.plays);
//
//
//				}

					//println!("value of returnwinner in second for loop, {:?}", returnwinner);
					//println!("value of oneaway {:?}", oneaway);
					//println!("value of twoaway {:?}", twoaway);
					//println!("value of threeaway {:?}", threeaway);
					//println!("value of fouraway {:?}", fouraway);


				if self.plays[iteratesecond].z == self.plays[iterate].z{

					secondpointx = self.plays[iteratesecond].x;
					secondpointy = self.plays[iteratesecond].y;
					
					calculation = ((firstpointx - secondpointx)*(firstpointx - secondpointx) + (firstpointy - secondpointy)*(firstpointy - secondpointy));
					calculationdummy = ((dummypointx - secondpointx)*(dummypointx - secondpointx) + (dummypointy - secondpointy)*(dummypointy - secondpointy));

					calculatedistance = (calculation as f64).sqrt();
					calculatedistancedummy = (calculationdummy as f64).sqrt();
				}


				if oneaway == false{

					//println!("We got to oneaway!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");


					if (calculatedistance == distfloat1 || calculatedistance == 1.0) && (calculatedistancedummy == distfloat1 || calculatedistancedummy == 1.0){ 


						dummypointx = secondpointx;
						dummypointy = secondpointy;
						oneaway = true;
						counter = 0;


					} else {
						if counter == 1 {
							oneaway = false;
							twoaway = false;
							threeaway = false;
							fouraway = false;
						} else {
							counter = 1;
						}

					}
					
				}else if twoaway == false{

					//println!("We got to TWOAWAY :::::::::::::::::::::::::::::::::::::::!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
					//println!("Here is self.plays[iterate] {:?}", self.plays[iterate]);
					//println!("Here is self.plays[iteratesecond] {:?}", self.plays[iteratesecond]);

					if (calculatedistance == distfloat2 || calculatedistance == 2.0) && (calculatedistancedummy == distfloat1 || calculatedistancedummy == 1.0){
						dummypointx = secondpointx;
						dummypointy = secondpointy;
						twoaway = true;
						counter = 0;
					} else {
						if counter == 1 {
							oneaway = false;
							twoaway = false;
							threeaway = false;
							fouraway = false;
						} else {
							counter = 1;
						}
					}
					
				}else if threeaway == false{



					//println!("We got to threeaway!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
				//	println!("Here is self.plays[iterate] {:?}", self.plays[iterate]);
			//		println!("Here is self.plays[iteratesecond] {:?}", self.plays[iteratesecond]);


					if (calculatedistance == distfloat3 || calculatedistance == 3.0) && (calculatedistancedummy == distfloat1 || calculatedistancedummy == 1.0){
						dummypointx = secondpointx;
						dummypointy = secondpointy;
						threeaway = true;
						counter = 0;
					} else {
						if counter == 1 {
							oneaway = false;
							twoaway = false;
							threeaway = false;
							fouraway = false;
						} else {
							counter = 1;
						}

					}
					
				}else if fouraway == false{


			//		println!("We got to fouraway!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
			//		println!("Here is the calculatedistance {:?}", calculatedistance);
			//		println!("Here is the calculatedistancedummy{:?}", calculatedistancedummy);


					if (calculatedistance == distfloat4 || calculatedistance == 4.0) && (calculatedistancedummy == distfloat1 || calculatedistancedummy == 1.0){	

						returnwinner = self.plays[iteratesecond].z.to_string();
			//			if self.plays.len() == 9{
			//				println!("value of returnwinner when it should be assigned {:?}", returnwinner);
			//			}
						dummypointx = secondpointx;
						dummypointy = secondpointy;
						fouraway = true;
						counter = 0;
					} else {
						if counter == 1 {
							oneaway = false;
							twoaway = false;
							threeaway = false;
							fouraway = false;
						} else {
							counter = 1;
						}

					}
					
				}else{
					//println!("This statement should never be evaluated!");
					break;
				}





				
			}
		
		}

	//	println!("value of returnwinner right before it is returned, {:?}", returnwinner);
	//	println!("value of oneaway {:?}", oneaway);
	//	println!("value of twoaway {:?}", twoaway);
	//	println!("value of threeaway {:?}", threeaway);
	//	println!("value of fouraway {:?}", fouraway);



		return returnwinner;


	}


}	



fn main() {

	//initial game print

    println!("Hello, and welcome!");
    println!("We are going to play PENTE");
    println!("For instructions on how to play, please see here (http://www.pente.net/instructions.html)");
    println!("Here is the game grid:");
    println!("");
    println!("    0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30");
    println!("0   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("1   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("2   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("3   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("4   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("5   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("6   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("7   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("8   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("9   .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("10  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("11  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("12  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("13  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("14  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("15  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("16  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("17  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("18  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("19  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("20  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("21  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("22  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("23  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("24  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("25  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("26  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("27  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("28  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("29  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");
    println!("30  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .  .");

    println!("");
    println!("In order to play, input what coordinate you would like to place a piece");


    //Here we declare a bunch of varialbes:

    let mut column = 31.0;
    let mut row = 31.0;
    let mut zvalue = String::new();
    let mut stack = Vec::new();
   	let mut newpoint = Point {
        x: column,
        y: row,
        z: zvalue,
    };
    let mut currentgame = GameTable {
    	plays : stack,
    };
    let mut fiveinarowwincheck = String::new();
    let mut playerstealnumber = 0;
    let mut computerstealnumber = 0;
    let mut playerorcomputermain = String::new();


    //this is the game loop

    loop{

	    //Here is the player input and test of input - loop until good


		loop{
		
		    println!("What COLUMN would you like your piece to go?:");


				let mut xcolumn_text = String::new();
		    	io::stdin().read_line(&mut xcolumn_text)
		        	.ok()
		        	.expect("Couldn't read line"); 
		        let mut xcolumn = xcolumn_text.trim();
		        match xcolumn.parse::<f64>() {
		        	Ok(i) => {column = i;
		        			  break;},
		        	Err(..) => {println!("this was not PROPER input: {}", xcolumn);
		        				println!("try again, but this time only input an integer between 0 and 30");}
		       	};
		}

		loop{
		    println!("What ROW would you like your piece to go?:");

				let mut yrow_text = String::new();
		    	io::stdin().read_line(&mut yrow_text)
		        	.ok()
		        	.expect("Couldn't read line"); 
		        let mut yrow = yrow_text.trim();
		        match yrow.parse::<f64>() {
		        	Ok(i) => {row = i;
		        			  break;},
		        	Err(..) => {println!("this was not PROPER input: {}", yrow);
		        			   println!("try again, but this time only input an integer between 0 and 30");}
		        };
		}


		//print out the varified/tested column and row of user input

		println!("Column, {:?}", column);
		println!("Row, {:?}", row);


		//update the stack for the players turn


		newpoint = Point{

			x: row,
			y: column,
			z: "player".to_string(),
		};

	    currentgame.plays.push(newpoint);


	    //print out the gametable after the players turn

	    currentgame.displaytable();

	    //check if the player won

	    println!("**********************************************************************************************");

	    fiveinarowwincheck = currentgame.check4fiveinarow();
	    println!("five in a row check is {:?}", fiveinarowwincheck);
	    println!("here is the current move stack {:?}", currentgame.plays);

	    println!("**********************************************************************************************");

	    if fiveinarowwincheck == "player"{
	    	println!("The player got five in a row!");
	    	println!("The player has won!");
	    	break;
	    }

	    playerstealnumber += currentgame.updatesteals();

	    if playerstealnumber == 5 {
	    	println!("The player stole five pairs!");
	    	println!("The player wins!");
	    	break;
	    }


	    //fn findunboundedchain(&mut self, playerorcomputer: String) -> (u32, u32){

	    playerorcomputermain = "player".to_string();
	    let unboundedchainplayer = currentgame.findunboundedchain(playerorcomputermain);


    	println!("the longest unboundedchain for the player is {:?}", unboundedchainplayer);


	    //now it's the computers turn

	    computerstealnumber += currentgame.computercontrol(computerstealnumber, playerstealnumber);

	    //display the game table after the computers turn

	    currentgame.displaytable();

	    //check if the computer won
		
		fiveinarowwincheck = currentgame.check4fiveinarow();

	    if fiveinarowwincheck == "computer"{
	    	println!("The computer got five in a row!");
	    	println!("The computer has won!");
	    	break;
	    }

	    if computerstealnumber == 5 {
	    	println!("The computer stole five pairs");
	    	println!("The computer has won!");
	    	break;
	    }

	}



}
