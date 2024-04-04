 mod linear_observation_model;
 mod motion_model;
 mod print_csv;

use nalgebra::{Matrix1x2, Matrix1x4, Matrix4, Vector2, Vector4};
use nalgebra_rand_mvn::rand_mvn;

use adskalman::{KalmanFilterNoControl, ObservationModel};

type MyType = f64;

// the main program --------

pub fn test222() -> Result<(), anyhow::Error> {
    env_logger::init();

    let dt = 0.01;
    let true_initial_state = Vector4::<MyType>::new(0.0, 0.0, 10.0, -5.0);
    #[rustfmt::skip]
    let initial_covariance = Matrix4::<MyType>::new(0.1, 0.0, 0.0, 0.0,
        0.0, 0.1, 0.0, 0.0,
        0.0, 0.0, 0.1, 0.0,
        0.0, 0.0, 0.0, 0.1);

    let motion_model = motion_model::ConstantVelocity2DModel::new(dt, 100.0);
    let observation_model = linear_observation_model::PositionObservationModel::new(0.01);
    let kf = KalmanFilterNoControl::new(&motion_model, &observation_model);

    // Create some fake data with our model.
    //TODO: connect x,y, vx,vy from headset.
    let mut current_state = true_initial_state;
    let mut state = vec![];
    let mut times = vec![];
    let zero4 = Vector4::<MyType>::zeros();
    let mut cur_time = 0.0;
    while cur_time < 0.5 {
        times.push(cur_time);
        state.push(current_state);
        let noise_sample: Matrix1x4<MyType> =
            rand_mvn(&zero4, motion_model.transition_noise_covariance).unwrap();
        let noise_sample_col: Vector4<MyType> = noise_sample.transpose();
        current_state = motion_model.transition_model * current_state + noise_sample_col;
        cur_time += dt;
    }

    // Create noisy observations.
    //TODO: connect observation
    let mut observation = vec![];
    let zero2 = Vector2::<MyType>::zeros();
    for current_state in state.iter() {
        let noise_sample: Matrix1x2<MyType> =
            rand_mvn(&zero2, observation_model.observation_noise_covariance).unwrap();
        let noise_sample_col = noise_sample.transpose();
        let current_observation =
            observation_model.predict_observation(current_state) + noise_sample_col;
        observation.push(current_observation);
    }

    let mut previous_estimate =
        adskalman::StateAndCovariance::new(true_initial_state, initial_covariance);

    let mut state_estimates = vec![];
    for this_observation in observation.iter() {
        let this_estimate = kf.step(&previous_estimate, this_observation)?;
        state_estimates.push(*this_estimate.state());
        previous_estimate = this_estimate;
    }
    print_csv::print_csv(&times, &state, &observation, &state_estimates);
    let write_sucess = print_csv::write_csv_hardcode(&times, &state, &observation, &state_estimates);
    match write_sucess {
        Ok(contents) => println!("File contents"),
        Err(error) => println!("Error writting file {} ",error ),
    }
    Ok(())
}

