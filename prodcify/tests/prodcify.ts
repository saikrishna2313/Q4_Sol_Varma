import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Prodcify } from "../target/types/prodcify";
import { Keypair, LAMPORTS_PER_SOL} from "@solana/web3.js";
import { keypairs } from "../keypairs";

describe("prodcify", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Prodcify as Program<Prodcify>;

  const user1 = Keypair.fromSecretKey(new Uint8Array(keypairs[0].privateKey));
  const user2 = Keypair.fromSecretKey(new Uint8Array(keypairs[1].privateKey));
    
  it("Create Profile!", async () => {
    const user = user1;

    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), user.publicKey.toBuffer()],
      program.programId
    );

    const [potPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pot"), user.publicKey.toBuffer(), profilePda.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .createProfile("Sai")
      .accounts({
        signer: user.publicKey,
        profile: profilePda,
        pot: potPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any).signers([user])
      .rpc();

    console.log("Transaction signature for Create Profile:", tx);

    const profileAccount = await program.account.profile.fetch(profilePda);
    console.log("Profile Account:", profileAccount);
  });

  it("Create Project!", async () => {
    const user = provider.wallet;

    const projectName = "Prodcify";

    const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectName)],
      program.programId
    );

    const tx = await program.methods
      .createProject(projectName)
      .accounts({
        signer: user.publicKey,
        project: projectPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    console.log("Transaction signature for Create Project:", tx);

    const projectAccount = await program.account.project.fetch(projectPda);
    console.log("Project Account:", projectAccount);
  });

  it("Create Team Member!", async () => {
    const user = provider.wallet;
    const projectName = "Prodcify";
    const teamMemberName = "Varma";

    const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectName)],
      program.programId
    );

    const [teamMemberPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("team_member"), Buffer.from(projectName), user1.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .createTeamMember(teamMemberName, projectName, user1.publicKey)
      .accounts({
        signer: user.publicKey,
        teamMember: teamMemberPda,
        project: projectPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    console.log("Transaction signature for Create Team Member:", tx);

    const teamMemberAccount = await program.account.teamMember.fetch(teamMemberPda);
    console.log("Team Member Account:", teamMemberAccount);
  });

  it.only("Create Task!", async () => {
    const user = provider.wallet;
    const projectName = "Prodcify";
    const taskName = "Task3";
    const deadline = Math.floor(Date.now() / 1000) + 3600;
   console.log(deadline)

    const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectName)],
      program.programId
    );

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("task"), Buffer.from(taskName), user.publicKey.toBuffer(), user1.publicKey.toBuffer(), Buffer.from(projectName)],
      program.programId
    );

    const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultA"), taskPda.toBuffer()],
      program.programId
    );

    const [vaultBPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultB"), vaultAPda.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .createTask(taskName, user1.publicKey, projectName, new anchor.BN(deadline))
      .accounts({
        signer: user.publicKey,
        task: taskPda,
        project: projectPda,
        vaultA: vaultAPda,
        vaultB: vaultBPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    console.log("Transaction signature for Create Task:", tx);

    const taskAccount = await program.account.task.fetch(taskPda);
    console.log("Task Account:", taskAccount);
  });


  it("Create Sub Task!", async () => {
    const projectName = "Prodcify";
  const taskName = "Task2";
  const subtaskName = "Subtask3";
  const points =40;
  const reward =1; 
  const user = provider.wallet;
  const mainTask=true;
  const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("project"), Buffer.from(projectName)],
    program.programId
  );

  const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("task"), Buffer.from(taskName), user.publicKey.toBuffer(), user1.publicKey.toBuffer(), Buffer.from(projectName)],
    program.programId
  );

  const [subtaskPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("subtask"), taskPda.toBuffer(), Buffer.from(subtaskName)],
    program.programId
  );

  const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vaultA"), taskPda.toBuffer()],
    program.programId
  );

  
  const tx = await program.methods
    .createSubtask(subtaskName, user1.publicKey, new anchor.BN(points), projectName, new anchor.BN(reward), taskName,mainTask)
    .accounts({
      signer: user.publicKey,
      task: taskPda,
      subtask: subtaskPda,
      project: projectPda,
      vaultA: vaultAPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    } as any)
    .rpc();

  console.log("Transaction signature for Create Subtask:", tx);
  const subtaskAccount = await program.account.subtask.fetch(subtaskPda);
  console.log("Subtask Account:");
  console.log({
    name: subtaskAccount.name,
    project: subtaskAccount.project,
    task: subtaskAccount.task.toString(),
    owner: subtaskAccount.owner.toString(),
    member: subtaskAccount.member.toString(),
    reward: subtaskAccount.reward.toNumber(),
    taskType: subtaskAccount.taskType,
    bump: subtaskAccount.bump,
    points: subtaskAccount.points.toNumber(),
    status: subtaskAccount.status,
  });
  
  const taskAccount = await program.account.task.fetch(taskPda);
  console.log("Task Account:");
  console.log({
    name: taskAccount.name,
    project: taskAccount.project,
    accepted: taskAccount.accepted,
    bump: taskAccount.bump,
    status: taskAccount.status,
    owner: taskAccount.owner.toString(), 
    member: taskAccount.member.toString(),
    completedSubtasks: taskAccount.completedSubtasks.toNumber(),
    totalSubtasks: taskAccount.totalSubtasks.toNumber(),
    mainSubtasks: taskAccount.mainSubtasks.toNumber(),
    mainSubtasksCompleted: taskAccount.mainSubtasksCompleted.toNumber(),
    startTime: taskAccount.startTime.toNumber(),
    deadline: taskAccount.deadline.toNumber(),
    vaultA: taskAccount.vaultA.toString(),
    vaultB: taskAccount.vaultB.toString(),
    vaultABump: taskAccount.vaultABump,
    vaultBBump: taskAccount.vaultBBump,
    subtasks: taskAccount.subtasks,
    points: taskAccount.points.toNumber(),
    totalReward: taskAccount.totalReward.toNumber(),
  });
  
  });
  it("Enroll Task!", async () => {
    const member = user1; 
    const projectName = "Prodcify";
    const taskName = "Task2";
    const user = provider.wallet;
  
  
    const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectName)],
      program.programId
    );
  
    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        Buffer.from(taskName),
        user.publicKey.toBuffer(),
        member.publicKey.toBuffer(), 
        Buffer.from(projectName),
      ],
      program.programId
    );
  
    
    const [teamMemberPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("team_member"), Buffer.from(projectName), member.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .enrollTask(projectName, taskName)
      .accounts({
        signer: member.publicKey,
        task: taskPda,
        project: projectPda,
        teamMember: teamMemberPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([member]) 
      .rpc();
  
    console.log("Transaction signature for Enroll Task:", tx);
  
    const taskAccount = await program.account.task.fetch(taskPda);
    console.log("Task Account after enrolling:", taskAccount);
  

    const teamMemberAccount = await program.account.teamMember.fetch(teamMemberPda);
    console.log("Team Member Account after enrolling:", teamMemberAccount);
  });
  

  it("Accept Subtask and Transfer Reward!", async () => {
    const projectName = "Prodcify";
    const taskName = "Task2";
    const subtaskName = "Subtask1";
    const user=provider.wallet;

    const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectName)],
      program.programId
    );
  
    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        Buffer.from(taskName),
        user.publicKey.toBuffer(),
        user1.publicKey.toBuffer(),
        Buffer.from(projectName),
      ],
      program.programId
    );
  
    const [subtaskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subtask"), taskPda.toBuffer(), Buffer.from(subtaskName)],
      program.programId
    );
  
    const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultA"), taskPda.toBuffer()],
      program.programId
    );
  
    const [vaultBPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultB"), vaultAPda.toBuffer()],
      program.programId
    );
  
  
    const vaultAInitialBalance = await provider.connection.getBalance(vaultAPda);
    const vaultBInitialBalance = await provider.connection.getBalance(vaultBPda);
  
    console.log(`Initial Vault A Balance: ${vaultAInitialBalance/LAMPORTS_PER_SOL}`);
    console.log(`Initial Vault B Balance: ${vaultBInitialBalance/LAMPORTS_PER_SOL}`);
  
    const tx = await program.methods
      .acceptSubtask(subtaskName,user1.publicKey,projectName,taskName)
      .accounts({
        signer: user.publicKey,
        task: taskPda,
        subtask: subtaskPda,
        vaultA: vaultAPda,
        vaultB: vaultBPda,
        project: projectPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }as any)
      .rpc();
  
    console.log("Transaction signature:", tx);
  
    const updatedTask = await program.account.task.fetch(taskPda);
    const updatedSubtask = await program.account.subtask.fetch(subtaskPda);
   console.log(`Task after Update:${updatedTask}`)
   console.log(`Subtask after Update:${updatedSubtask}`)
    const vaultAFinalBalance = await provider.connection.getBalance(vaultAPda);
    const vaultBFinalBalance = await provider.connection.getBalance(vaultBPda);
  
    console.log(`Final Vault A Balance: ${vaultAFinalBalance/LAMPORTS_PER_SOL}`);
    console.log(`Final Vault B Balance: ${vaultBFinalBalance/LAMPORTS_PER_SOL}`);
  
  });
  


  it("Accept Task and Transfer Funds!", async () => {
    const projectName = "Prodcify";
    const taskName = "Task1";
    const user=provider.wallet;
    const member=user1;

    const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectName)],
      program.programId
    );
  
    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        Buffer.from(taskName),
        user.publicKey.toBuffer(),
        member.publicKey.toBuffer(),
        Buffer.from(projectName),
      ],
      program.programId
    );
  
    const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultA"), taskPda.toBuffer()],
      program.programId
    );
  
    const [vaultBPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultB"), vaultAPda.toBuffer()],
      program.programId
    );
  
    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), member.publicKey.toBuffer()],
      program.programId
    );
    const [teamMemberPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("team_member"), Buffer.from(projectName), member.publicKey.toBuffer()],
      program.programId
    );
    const [potPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pot"), member.publicKey.toBuffer(), profilePda.toBuffer()],
      program.programId
    );
  
    const vaultAInitialBalance = await provider.connection.getBalance(vaultAPda);
    const vaultBInitialBalance = await provider.connection.getBalance(vaultBPda);
    const profilePotInitialBalance = await provider.connection.getBalance(potPda);
    const memberInitialBalance = await provider.connection.getBalance(member.publicKey);
  
    console.log(`Initial Vault A Balance: ${vaultAInitialBalance}`);
    console.log(`Initial Vault B Balance: ${vaultBInitialBalance}`);
    console.log(`Initial Profile Pot Balance: ${profilePotInitialBalance}`);
    console.log(`Initial Member Balance: ${memberInitialBalance}`);

    const tx = await program.methods
      .acceptTask(taskName, member.publicKey, projectName)
      .accounts({
        signer: user.publicKey,
        task: taskPda,
        vaultA: vaultAPda,
        vaultB: vaultBPda,
        project: projectPda,
        teamMember: teamMemberPda,
        profile: profilePda,
        pot: potPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();
  
    console.log("Transaction signature:", tx);
  
    
    const vaultAFinalBalance = await provider.connection.getBalance(vaultAPda);
    const vaultBFinalBalance = await provider.connection.getBalance(vaultBPda);
    const profilePotFinalBalance = await provider.connection.getBalance(potPda);
    const memberFinalBalance = await provider.connection.getBalance(member.publicKey);
  
    console.log(`Final Vault A Balance: ${vaultAFinalBalance}`);
    console.log(`Final Vault B Balance: ${vaultBFinalBalance}`);
    console.log(`Final Profile Pot Balance: ${profilePotFinalBalance}`);
    console.log(`Final Member Balance: ${memberFinalBalance}`);

  });
    
  it("Member Exit Task", async () => {
    const user = provider.wallet; 
    const member=user1;
    const projectName = "Prodcify";
    const taskName = "Task2";
  
    const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectName)],
      program.programId
    );
  
    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        Buffer.from(taskName),
        user.publicKey.toBuffer(),
        member.publicKey.toBuffer(),
        Buffer.from(projectName),
      ],
      program.programId
    );
  
    const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultA"), taskPda.toBuffer()],
      program.programId
    );
  
    const [vaultBPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultB"), vaultAPda.toBuffer()],
      program.programId
    );
  
    const [teamMemberPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("team_member"), Buffer.from(projectName), member.publicKey.toBuffer()],
      program.programId
    );
  
    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), member.publicKey.toBuffer()],
      program.programId
    );
  
    const [potPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pot"),member.publicKey.toBuffer(),profilePda.toBuffer()],
      program.programId
    );

const vaultBBalanceBefore = await provider.connection.getBalance(vaultBPda);
console.log("Vault B Balance Before:", vaultBBalanceBefore/LAMPORTS_PER_SOL);

const vaultABalanceBefore = await provider.connection.getBalance(vaultAPda);
console.log("Vault A Balance Before:", vaultABalanceBefore/LAMPORTS_PER_SOL);

const potBalanceBefore = await provider.connection.getBalance(potPda);
console.log("Pot Balance Before:", potBalanceBefore/LAMPORTS_PER_SOL);

    const tx = await program.methods
      .memberExitTask(taskName, projectName)
      .accounts({
        signer: member.publicKey,
        task: taskPda,
        vaultA: vaultAPda,
        vaultB: vaultBPda,
        project: projectPda,
        teamMember: teamMemberPda,
        profile: profilePda,
        pot: potPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }as any)
      .signers([member])
      .rpc();
  
    console.log("Transaction signature for Member Exit Task:", tx);
  
const vaultBBalanceAfter = await provider.connection.getBalance(vaultBPda);
console.log("Vault B Balance After:", vaultBBalanceAfter/LAMPORTS_PER_SOL);

const vaultABalanceAfter = await provider.connection.getBalance(vaultAPda);
console.log("Vault A Balance After:", vaultABalanceAfter/LAMPORTS_PER_SOL);

const potBalanceAfter = await provider.connection.getBalance(potPda);
console.log("Pot Balance After:", potBalanceAfter/LAMPORTS_PER_SOL);  
    
  });

it("Delete  Task", async () => {
    const user = provider.wallet; 
    const member=user1;
    const projectName = "Prodcify";
    const taskName = "Task2";
  
    const [projectPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectName)],
      program.programId
    );
  
    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("task"),
        Buffer.from(taskName),
        user.publicKey.toBuffer(),
        member.publicKey.toBuffer(),
        Buffer.from(projectName),
      ],
      program.programId
    );
  
    const [vaultAPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultA"), taskPda.toBuffer()],
      program.programId
    );
  
    const [vaultBPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vaultB"), vaultAPda.toBuffer()],
      program.programId
    );
  
    const [teamMemberPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("team_member"), Buffer.from(projectName), member.publicKey.toBuffer()],
      program.programId
    );
  
    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), member.publicKey.toBuffer()],
      program.programId
    );
  
    const [potPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pot"),member.publicKey.toBuffer(),profilePda.toBuffer()],
      program.programId
    );

const vaultBBalanceBefore = await provider.connection.getBalance(vaultBPda);
console.log("Vault B Balance Before:", vaultBBalanceBefore/LAMPORTS_PER_SOL);

const vaultABalanceBefore = await provider.connection.getBalance(vaultAPda);
console.log("Vault A Balance Before:", vaultABalanceBefore/LAMPORTS_PER_SOL);

const potBalanceBefore = await provider.connection.getBalance(potPda);
console.log("Pot Balance Before:", potBalanceBefore/LAMPORTS_PER_SOL);

    const tx = await program.methods
      .removeTask(taskName,member.publicKey, projectName)
      .accounts({
        signer: user.publicKey,
        task: taskPda,
        vaultA: vaultAPda,
        vaultB: vaultBPda,
        project: projectPda,
        teamMember: teamMemberPda,
        profile: profilePda,
        pot: potPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }as any)
      .rpc();
  
    console.log("Transaction signature for Member Exit Task:", tx);
  
const vaultBBalanceAfter = await provider.connection.getBalance(vaultBPda);
console.log("Vault B Balance After:", vaultBBalanceAfter/LAMPORTS_PER_SOL);

const vaultABalanceAfter = await provider.connection.getBalance(vaultAPda);
console.log("Vault A Balance After:", vaultABalanceAfter/LAMPORTS_PER_SOL);

const potBalanceAfter = await provider.connection.getBalance(potPda);
console.log("Pot Balance After:", potBalanceAfter/LAMPORTS_PER_SOL);  
    
  });
  
  it("Withdraw Funds", async () => {
   
    const member=user1;
  
    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), member.publicKey.toBuffer()],
      program.programId
    );
  
    const [potPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pot"),member.publicKey.toBuffer(),profilePda.toBuffer()],
      program.programId
    );

const potBalanceBefore = await provider.connection.getBalance(potPda);
console.log("Pot Balance Before:", potBalanceBefore/LAMPORTS_PER_SOL);

    const tx = await program.methods
      .withdrawFunds()
      .accounts({
        signer: member.publicKey,
        profile: profilePda,
        pot: potPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }as any)
      .signers([member])
      .rpc();
  
console.log("Transaction signature for Amount Withdrwan:", tx);
const potBalanceAfter = await provider.connection.getBalance(potPda);
console.log("Pot Balance After:", potBalanceAfter/LAMPORTS_PER_SOL);  
    
  });
});
