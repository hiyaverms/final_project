
Project Goal: Do certain actors link groups of actors that work together? Do certain actors have significantly more connections to other actors than is average? Who are the most well-connected actors?


Dataset: I used IMDb’s non-commercial datasets. Specifically, I used name.basics.tsv.gz and title.principals.tsv.gz. My cleaned  data is here: https://drive.google.com/drive/folders/19m2i4oNvlRNCXcam_27tmER4-NjLS0Uw?usp=drive_link .


 Data Processing


I used title.principles to get everyone who worked on every title on IMDb. I used Pandas to filter out only the actors. Then I extracted pairs of unique alphanumeric identifiers for each title and an alphanumeric identifier for an actor who worked on that movie. I used name.basics to match the unique alphanumeric identifier for the actors to their primary name. If any of the information I needed was missing, I removed them from the dataset. In the end, I had three columns and around 18 million lines. The first column is the unique identifier for the actor, the second column is their name, and the third line is the unique identifier of the title. An actor could have several rows if they had been in several titles.
I downloaded the cleaned data and uploaded it to MOS. 


Code Structure
Modules


I decided to create three modules: parser.rs, graph.rs, and main.rs. I used the parser to just make the data comprehensible so I could later construct the graph. graph.rs had the bulk of the project. It had all the functions needed to do analysis on the graph, as well as tests. In main.rs, I utilized these functions to actually look at some interesting parts of the graph and try to answer the project question. 




Key Functions & Types (Structs, Enums, Traits, etc)

I created a struct Record that represented one line of the dataset. It was helpful in parsing the file.
I used function degree_centrality to see how many connections an actor had. It was computed by counting how many nodes a given node was connected to. It takes a reference to the graph and returns a hashmap that links the node to the number of neighbors it has. 
I used function closeness_centrality to find how easily an actor could connect with another actor. It was computed by finding the shortest path to another node using Dijkstra’s algorithm. It takes a reference to the graph and returns a hashmap that maps each node to its closeness centrality score.  
I used function betweeness_centrality to see if any actors connect clusters of actors who often work together, such as Marvel actors. It was computed using breadth first search. It takes a reference to the graph and returns a hashmap that maps each node to its betweenness centrality score.  
I used function shortest_path_length to find how easily two actors who are in different realms could be connected. It was computed using Dijkstra’s algorithm. It takes a reference to the graph, the id of the node you are starting from, and the id of the node you are going to. It returns the length of the path between them if one exists. 



Main Workflow


main.rs calls parser.rs to parse the file. Then, build_graph from graph.rs is used to build the undirected graph. Then a hashmap is created that maps each actor id to a node index. For my first example, I wanted to look at Priyanka Chopra Jonas, so a subgraph is created centered on her. Each centrality measure is run on the subgraph by using the functions in graph.rs described above. I then looked at a random sample of 500 actors and ran the centrality measures again. I was also interested in how many connections away a major Bollywood star could be from a major Hollywood star. I chose the grandfather of Bollywood, Amitabh Bachan, and a current Hollywood star, Timothée Chalamet. I used shortest_path_length from graph.rs to find this. 




Tests
cargo test output 
Finished `test` profile [unoptimized + debuginfo] target(s) in 1.77s
     Running unittests src/main.rs (target/debug/deps/final_project-246f4b6cad6078ff)

running 8 tests
test graph::tests::test_build_graph ... ok
test graph::tests::test_closeness_centrality ... ok
test graph::tests::test_connected_components ... ok
test graph::tests::test_connected_components_map ... ok
test graph::tests::test_degree_centrality ... ok
test graph::tests::test_extract_subgraph_around_actor ... ok
test graph::tests::test_shortest_path_length ... ok
test graph::tests::test_random_actor_subgraph ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


Tests:
The first test just ensures that the graph itself is being built correctly.
The second test makes sure that my function for degree centrality has correct logic.
The third test made sure I correctly defined a component, which is essentially a group of actors.
The fourth test makes sure the corresponding grouping was happening correctly.
The fifth test makes sure that my function for closeness centrality has correct logic.
The sixth test makes sure that I was able to accurately build a graph centered on a given node.
The seventh test validates that I am able to make a graph from a random sample of the given size.
The eighth test makes sure that my function for betweenness centrality has correct logic.







Results
Program Outputs:
Subgraph contains 380 nodes and 380 edges.

Top 10 Degree Centrality:
Priyanka Chopra Jonas              : 380  
Nathan Darrow                      : 1    
Ganesh Acharya                     : 1    
Dayashankar Pandey                 : 1    
Yashpal Sharma                     : 1    
Julia Louis-Dreyfus                : 1    
Steve Oram                         : 1    
Eric Bauza                         : 1    
Anupam Kher                        : 1    
Ankur Nayyar                       : 1    

Top 10 Closeness Centrality:
Priyanka Chopra Jonas              : 1.000
Carrie-Anne Moss                   : 0.501
Rajesh Khera                       : 0.501
Shiney Ahuja                       : 0.501
Rahul Bose                         : 0.501
Boman Irani                        : 0.501
Shahbaaz Khan                      : 0.501
Nikki Amuka-Bird                   : 0.501
Alan Powell                        : 0.501
Ashish Vidyarthi                   : 0.501

Top 10 Betweenness Centrality:
Priyanka Chopra Jonas              : 2.000
Kareena Kapoor                     : 0.000
Deb Mukherjee                      : 0.000
Payal Rohatgi                      : 0.000
Tanay Chheda                       : 0.000
Raza Murad                         : 0.000
Farhan Akhtar                      : 0.000
Govinda                            : 0.000
Prithviraj Sukumaran               : 0.000
John Cena                          : 0.000
Sampled subgraph has 500 nodes and 137 edges.

Top 10 Degree Centrality on Sample:
Sheryl Lee Ralph                    4
Karen Malina White                  3
Katarzyna Cichopek                  3
Sue Johnston                        3
Franco Del Torro                    3
Hasibe Eren                         3
Rena Kuroki                         3
Ilknur Boyraz                       3
Holly Aird                          3
Rafael Gareisen                     3

Top 10 Closeness Centrality on Sample:
Tresy Taddei                        1.000
Colin Delaney                       1.000
Cliff Compton                       1.000
Benjamin Berger                     1.000
Rafael Gareisen                     1.000
Angelene Aguilar                    1.000
Bettina Banoun                      1.000
H. Richard Greene                   1.000
Anna von Haebler                    1.000
Zuleyka Rivera Mendoza              1.000

Top 10 Betweenness Centrality on Sample:
Sheryl Lee Ralph                    0.000
Karen Malina White                  0.000
Katarzyna Cichopek                  0.000
Erdal Besikçioglu                   0.000
Hasibe Eren                         0.000
Franco Del Torro                    0.000
Ilknur Boyraz                       0.000
Rena Kuroki                         0.000
Rafael Gareisen                     0.000
Jean Alexander                      0.000

Shortest Path from Amitabh Bachchan to Timothée Chalamet:
Shortest path length from Amitabh Bachchan to Timothée Chalamet: 2



Interpretation in project context (no need for "groundbreaking” results).
The first graph is a subgraph centered on Priyanka Chopra Jonas with depth 1 because it got a little too crazy for the scope of this project when it was increased. It had 380 nodes and 380 edges, meaning these actors were not really connected to each other. This also explains why in closeness centrality they were all two edges away (them to Priyanka, Priyanka to whoever else). This pattern holds for betweenness centrality. 
The second graph is a random selection of 500 actors. I created this subgraph in hopes of finding more meaningful results from the centrality measures. This graph is also sparse though, there are 500 nodes and 137 edges. The top actors of this sample had a degree centrality of 4, which means they had not acting with many other people, which explains the sparse graph. However, it seems there were some clusters, as some people had a closeness centrality of 1. The betweenness centrality is 0 for all of them, meaning none of them lie on the shortest path to another actor. This actually makes sense because this is a massive dataset that spans different genres, languages, and countries. When choosing such a relatively small number, there is not guarantee that the actors will be related. 
The last part looked at the shortest path between Amitabh Bachan and Timothée Chalamet. The path had a length of 2, meaning that someone who was in a movie with a current Hollywood star was also in a movie with the grandfather of Bollywood. 




 Usage Instructions
This must be run using –release. It takes far too long otherwise. The typical runtime is under 2 minutes when using –release. 
